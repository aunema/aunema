pub mod provider;
pub mod publisher;

use crate::config::Config;
use crate::helpers::database;

use actix_web::{web, App, HttpServer};

pub fn init_services(cnfg: Config) {
    let _conn = database::init_database(&cnfg, 5).expect("Failed to init database connection");

    // conn.execute("INSERT INTO videos (created_at) VALUES (1000), (2500)", &[])
    //     .expect("Failed while insert");
    // for row in &conn
    //     .query("SELECT id, created_at FROM videos", &[])
    //     .expect("Failed while select")
    // {
    //     let id: i32 = row.get(0);
    //     let created_at: i64 = row.get(1);
    //     println!("{} {}", id, created_at);
    // }

    println!("{:?}", cnfg);

    let app = move || {
        let _provider_dlr_cron = provider::delivery::cron::init(&cnfg);
        let provider_dlr_http = provider::delivery::http::init(&cnfg);

        App::new()
            .service(
                web::scope("/api").service(web::resource("*").to(|| "Api endpoint"))
            )
            .service(
                provider_dlr_http
            )
    };

    HttpServer::new(app)
        .bind("0.0.0.0:8000").expect("Failed to bind port for the http server")
        .run().expect("Failed to run http server");
}
