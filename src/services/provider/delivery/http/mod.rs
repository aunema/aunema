use crate::config::Config;

use actix_web::{web, App, HttpRequest, HttpResponse};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct ProviderHttp {
    pub cnfg: Arc<Config>,
}

pub fn init(cnfg: Arc<Config>) {
    let provider = ProviderHttp {
        cnfg: cnfg,
    };
    let data = web::Data::new(Mutex::new(provider));
    App::new()
        .data(data.clone())
        .service(web::resource("/").to(index));
}

fn index(state: web::Data<Mutex<Config>>, req: HttpRequest) -> HttpResponse {
    let state = state.lock().unwrap();
    println!("{:?}", req);
    HttpResponse::Ok().body(format!("Server port is {}", state.server_port))
}
