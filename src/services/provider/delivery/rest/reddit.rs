use actix_web::{web, HttpResponse, Scope};

pub fn init_endpoints() -> Scope {
    web::scope("/reddit").route("/fetch", web::get().to(fetch))
}

pub fn fetch(data: web::Data<super::ProviderRest>) -> HttpResponse {
    data.provider_cnr.fetch_reddit_posts();
    let res = json::object! { "success" => true }.dump();
    HttpResponse::Ok().body(res)
}
