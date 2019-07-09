pub mod provider;

use crate::config::Config;
use crate::helpers::database;
use crate::helpers::{email, handler};

use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{http, web, App, HttpServer};
use std::sync::Arc;

pub fn init_services(cnfg: Arc<Config>) {
    let port = cnfg.server_port;

    let db_pool = database::init_pool(&cnfg, 5).expect("Failed to init database connection");
    let mailer = email::init_mailer(&cnfg).expect("Failed to init mailer");

    let provider_ucs = provider::usecase::init(&cnfg, &db_pool);

    let provider_cnr = provider::controller::init(&cnfg, &provider_ucs, &mailer);

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

    HttpServer::new(app)
        .bind(format!("0.0.0.0:{}", port))
        .expect("Failed to bind port for the http server")
        .run()
        .expect("Failed to run http server");
}
