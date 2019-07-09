use crate::models::SocialNetwork;
use crate::helpers::handler;

use actix_web::{web, HttpResponse, Scope};

pub fn init_endpoints() -> Scope {
    web::scope("/reddit").route("/fetch", web::get().to(fetch))
}

pub fn fetch(data: web::Data<super::ProviderRest>) -> HttpResponse {
    let result = data.provider_cnr.fetch_media(SocialNetwork::Reddit);
    handler::to_json(result)
}
