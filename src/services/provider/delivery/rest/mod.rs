mod reddit;
mod links;

use crate::config::Config;
use crate::services::provider::controller::ProviderController;

use actix_web::{web, HttpResponse, Scope};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProviderRest {
    pub cnfg: Arc<Config>,
    pub provider_cnr: Arc<ProviderController>,
}

pub fn init(cnfg: &Arc<Config>, provider_cnr: &Arc<ProviderController>) -> Scope {
    let provider = ProviderRest {
        cnfg: cnfg.clone(),
        provider_cnr: provider_cnr.clone(),
    };
    web::scope("/provider")
        .data(provider)
        .route("/email", web::get().to(send_mail))
        .service(reddit::init_endpoints())
        .service(links::init_endpoints())
}

#[derive(Debug, Serialize, Deserialize)]
struct SendMailParams {
    email: String,
    template_id: String,
}

fn send_mail(params: web::Query<SendMailParams>, data: web::Data<ProviderRest>) -> HttpResponse {
    data.provider_cnr
        .send_mail(params.email.clone(), params.template_id.clone());
    HttpResponse::Ok().body("Mail sent")
}
