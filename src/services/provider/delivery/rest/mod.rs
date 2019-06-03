use crate::config::Config;
use crate::services::provider::controller::ProviderController;

use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug)]
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
        .route("/send", web::get().to(send_mail))
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
