use crate::config::Config;

use actix_web::{web, App, HttpRequest, HttpResponse};

use std::sync::Mutex;

pub fn init(cnfg: &Config) {
    let data = web::Data::new(Mutex::new(cnfg.clone()));

    // Todo: Fix it with static lifetime
    App::new()
        .data(data.clone())
        .service(web::resource("/").to(index));
}

fn index(state: web::Data<Mutex<Config>>, req: HttpRequest) -> HttpResponse {
    let state = state.lock().unwrap();
    println!("{:?}", req);
    HttpResponse::Ok().body(format!("Server port is {}", state.server_port))
}
