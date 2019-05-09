use crate::config::Config;

use actix_web::web;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct ProviderHttp<'a> {
    pub cnfg: &'a Config,
}

pub fn init(cnfg: &Config) {
    let provider = ProviderHttp { cnfg };
    let _data = web::Data::new(Mutex::new(provider));
}

// fn index(state: web::Data<Mutex<Config>>, req: HttpRequest) -> HttpResponse {
//     let state = state.lock().unwrap();
//     println!("{:?}", req);
//     HttpResponse::Ok().body(format!("Server port is {}", state.server_port))
// }
