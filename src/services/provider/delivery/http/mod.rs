use crate::config::Config;

use actix_web::{web, App, HttpRequest, HttpResponse};

pub struct ProviderHttp<'a> {
    pub cnfg: &'a Config,
}

impl<'a> Clone for ProviderHttp<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

pub fn init(cnfg: &Config) {
    let provider = ProviderHttp{cnfg:cnfg};

    // Todo: Fix it with static lifetime
    App::new()
        .data(&provider)
        .service(web::resource("/").to(index));
}

fn index(state: web::Data<ProviderHttp>, req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    HttpResponse::Ok().body(format!("Server port is {}", state.cnfg.server_port))
}
