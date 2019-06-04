pub mod provider;

use crate::config::Config;
use crate::helpers::database;
use crate::helpers::email;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;

pub fn init_services(cnfg: Arc<Config>) {
    let db_pool = database::init_pool(&cnfg, 5).expect("Failed to init database connection");
    let mailer = email::init_mailer(&cnfg).expect("Failed to init mailer");

    let provider_ucs = provider::usecase::init(&cnfg, &db_pool);

    let provider_cnr = provider::controller::init(&cnfg, &provider_ucs, &mailer);

    let app = move || {
        let provider_dlr_rest = provider::delivery::rest::init(&cnfg, &provider_cnr);

        let api = web::scope("/api/v1")
            .service(provider_dlr_rest);

        App::new().service(api)
    };

    // Todo: Move to main file
    HttpServer::new(app)
        .bind("0.0.0.0:8000")
        .expect("Failed to bind port for the http server")
        .run()
        .expect("Failed to run http server");
}
