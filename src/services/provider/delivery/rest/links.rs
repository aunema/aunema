use crate::models::SocialNetwork;
use crate::helpers::handler;

use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

pub fn init_endpoints() -> Scope {
    web::scope("/links")
        .route("/add", web::post().to(add_link))
        .route("/all", web::get().to(get_links))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddLinkBody {
    data: String,
    social_network: SocialNetwork,
}

pub fn add_link(
    body: web::Json<AddLinkBody>,
    data: web::Data<super::ProviderRest>,
) -> HttpResponse {
    let result = data
        .provider_cnr
        .add_link(body.data.clone(), body.social_network.clone());
    match result {
        Ok(value) => {
            let res = json::object! { "success" => true, "data" => json::parse(&serde_json::to_string(&value).expect("msg: &str")).expect("msg: &str") }.dump();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(res)
        }
        Err(err) => {
            let res = json::object! { "success" => false, "error" => err.to_string() }.dump();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(res)
        }
    }
}

// Todo: Fix deserialization errors to send as json
// Todo: Add default values

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetLinksParams {
    social_network: SocialNetwork,
    #[validate(range(min = "1", max = "50"))]
    #[serde(default)]
    limit: u32,
    #[serde(default)]
    offset: u32,
}

pub fn get_links(
    body: web::Query<GetLinksParams>,
    data: web::Data<super::ProviderRest>,
) -> HttpResponse {
    match body.validate() {
        Ok(_) => (),
        Err(e) => {
            let mut errors: Vec<String> = vec![];
            for (key, value) in &e.field_errors() {
                for inner in value {
                    errors.push(format!("Field {} failed with {} error", key, inner.code));
                }
            };
            return handler::handle(Err(errors));
        }
    };
    let result = data.provider_cnr.get_links(
        body.social_network.clone(),
        body.limit.clone(),
        body.offset.clone(),
    );
    match result {
        Ok(value) => {
            handler::handle(Ok(json::parse(&serde_json::to_string(&value).expect("msg: &str")).expect("msg: &str")))
        }
        Err(err) => {
            handler::handle(Err(vec![err.to_string()]))
        }
    }
}
