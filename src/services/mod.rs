use crate::config::Config;
use actix_web::{server, App, HttpRequest, Responder};

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", to)
}

pub fn init_services(cnfg: &Config) {
    println!("{:?}", cnfg);

    server::new(|| {
        App::new()
            .resource("/", |res| res.f(greet))
            .resource("/{name}", |res| res.f(greet))
    })
    .bind(format!("0.0.0.0:{}", cnfg.server_port))
    .expect(&format!("Can not bind to port {}", cnfg.server_port))
    .run();
}
