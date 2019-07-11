mod api;
mod image;
mod links;

use crate::config::Config;
use crate::helpers::email::Mailer;
use crate::models::SocialNetwork;
use crate::services::provider::usecase::ProviderUsecase;

use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
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

    pub fn fetch_media(&self, social_network: SocialNetwork) -> Result<u64, Box<dyn Error>> {
        let mut media = self.fetch_data(social_network)?;
        let uids: Vec<String> = media
            .iter()
            .map(|value| value.unique_identifier.clone())
            .collect();
        let existing_media = self.provider_ucs.get_media_by_uids(uids)?;

        media.retain(|media_item| {
            match existing_media
                .iter()
                .find(|item| item.unique_identifier == media_item.unique_identifier)
            {
                Some(_) => false,
                None => true,
            }
        });
        if media.len() <= 0 {
            return Err(Box::from("Fetch returned only repeats"));
        }

        let count = self.provider_ucs.add_media(media)?;
        Ok(count)
    }
}
