use crate::helpers::handler;
use crate::models::SocialNetwork;

use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

pub fn init_endpoints() -> Scope {
    web::scope("/links")
        .route("/add", web::post().to(add_link))
        .route("/remove", web::delete().to(remove_link))
        .route("/all", web::get().to(get_links))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AddLinkBody {
    #[validate(length(min = "3", max = "300"))]
    provider: String,
    #[validate(range(min = "1", max = "100"))]
    limit: i16,
    social_network: SocialNetwork,
}

pub fn add_link(
    body: web::Json<AddLinkBody>,
    data: web::Data<super::ProviderRest>,
) -> HttpResponse {
    crate::validate_errors!(body);
    let result = data.provider_cnr.add_link(
        body.provider.clone(),
        body.limit.clone(),
        body.social_network.clone(),
    );
    handler::to_json(result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveLinkBody {
    id: uuid::Uuid,
}

pub fn remove_link(
    body: web::Json<RemoveLinkBody>,
    data: web::Data<super::ProviderRest>,
) -> HttpResponse {
    let result = data.provider_cnr.remove_link(body.id.clone());
    handler::to_json(result)
}

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
    params: web::Query<GetLinksParams>,
    data: web::Data<super::ProviderRest>,
) -> HttpResponse {
    crate::validate_errors!(params);
    let result = data.provider_cnr.get_links(
        params.social_network.clone(),
        params.limit.clone(),
        params.offset.clone(),
    );
    handler::to_json(result)
}
