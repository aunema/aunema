use crate::helpers::handler;
use crate::models::SocialNetwork;

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
    handler::to_json(result)
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
    crate::handle_errors!(body);
    let result = data.provider_cnr.get_links(
        body.social_network.clone(),
        body.limit.clone(),
        body.offset.clone(),
    );
    handler::to_json(result)
}
