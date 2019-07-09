mod image;
mod links;

use crate::config::Config;
use crate::helpers::email::Mailer;
use crate::models::SocialNetwork;
use crate::services::provider::usecase::ProviderUsecase;

use std::error::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ProviderController {
    pub cnfg: Arc<Config>,
    pub provider_ucs: Arc<ProviderUsecase>,
    pub mailer: Arc<Mailer>,
}

pub fn init(
    cnfg: &Arc<Config>,
    provider_ucs: &Arc<ProviderUsecase>,
    mailer: &Arc<Mailer>,
) -> Arc<ProviderController> {
    let cnr = ProviderController {
        cnfg: cnfg.clone(),
        provider_ucs: provider_ucs.clone(),
        mailer: mailer.clone(),
    };
    Arc::new(cnr)
}

impl ProviderController {
    pub fn send_mail(&self, email: String, template_id: String) {
        self.mailer
            .send(email, template_id)
            .expect("Failed to send mail");
    }

    pub fn fetch_reddit_posts(&self) -> Result<(), Box<dyn Error>> {
        let links = self.provider_ucs.get_links(SocialNetwork::Reddit, 5, 0)?;
        for link in links {
            println!("Link: {:?}", link.data);
            // Todo: Fetch post image/animated/video
        }
        Ok(())
    }
}
