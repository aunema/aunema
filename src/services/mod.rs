pub mod provider;

use crate::config::Config;
use crate::helpers::{api, database, email, handler};

use actix::System;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{http, web, App, HttpServer};
use carapax::prelude::{UpdateMethod, UpdatesStream};
use std::sync::Arc;

pub fn init_services(cnfg: Arc<Config>) {
    let addr = format!("0.0.0.0:{}", cnfg.server_port);
    let token = cnfg.telegram_token.clone();

    let sys = System::new("aunema");
    let db_pool = database::init_pool(&cnfg, 5).expect("Failed to init database connection");
    let mailer = email::init_mailer(&cnfg).expect("Failed to init mailer");
    let mut telegram = api::init_telegram(token).expect("Failed to init telegram api");

    let provider_ucs = provider::usecase::init(&cnfg, &db_pool);

    let provider_cnr = provider::controller::init(&cnfg, &provider_ucs, &mailer);

    telegram.app = provider::delivery::telegram::init(&cnfg, &provider_cnr, telegram.app);
    let app = move || {
        let provider_dlr_rest = provider::delivery::rest::init(&cnfg, &provider_cnr);

        let api = web::scope("/api/v1").service(provider_dlr_rest);

        App::new()
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::BAD_REQUEST, handler::bad_request_handler),
            )
            .service(api)
    };

    actix::spawn(telegram.app.run(
        telegram.api.clone(),
        UpdateMethod::poll(UpdatesStream::new(telegram.api.clone())),
    ));
    HttpServer::new(app)
        .bind(addr)
        .expect("Failed to bind port for the http server")
        .start();
    sys.run().expect("Failed to run actix system");
}
