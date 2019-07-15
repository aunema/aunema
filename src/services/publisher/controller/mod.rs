use crate::config::Config;
use crate::models::{MediaType, SocialNetwork};
use crate::services::publisher::usecase::PublisherUsecase;

use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct PublisherController {
    pub cnfg: Arc<Config>,
    pub publisher_ucs: Arc<PublisherUsecase>,
}

pub fn init(cnfg: &Arc<Config>, publisher_ucs: &Arc<PublisherUsecase>) -> Arc<PublisherController> {
    let cnr = PublisherController {
        cnfg: cnfg.clone(),
        publisher_ucs: publisher_ucs.clone(),
    };
    Arc::new(cnr)
}

impl PublisherController {
    pub fn add_publisher(
        &self,
        chat_id: i64,
        social_network: SocialNetwork,
        supported_media: Vec<MediaType>,
        caption: Option<String>,
        repeats: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        // Todo: Check if publisher already exists
        // Todo: Add publisher
        Ok(())
    }
}
