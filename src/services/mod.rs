pub mod provider;
pub mod publisher;

use crate::config::Config;
use crate::helpers::database;

use actix_web::{HttpServer, App, HttpRequest, Responder};

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", to)
}

pub fn init_services<'a>(cnfg: &'a Config) {
    let conn = database::init_database(&cnfg, 5).expect("Failed to init database");

    conn.execute("INSERT INTO videos (created_at) VALUES (1000), (2500)", &[])
        .expect("Failed while insert");
    for row in &conn
        .query("SELECT id, created_at FROM videos", &[])
        .expect("Failed while select")
    {
        let id: i32 = row.get(0);
        let created_at: i64 = row.get(1);
        println!("{} {}", id, created_at);
    }

    println!("{:?}", cnfg);

    /*let providerDlrHttp = */provider::delivery::http::init(&cnfg);

    //move is necessary to give closure below ownership of counter
    // HttpServer::new(move || {
    // })
    // .bind(format!("0.0.0.0:{}", cnfg.server_port))
    // .expect(&format!("Can not bind to port {}", cnfg.server_port))
    // .run()
}
