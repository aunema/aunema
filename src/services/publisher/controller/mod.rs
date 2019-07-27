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
        _chat_id: i64,
        _social_network: SocialNetwork,
        _supported_media: Vec<MediaType>,
        _caption: Option<String>,
        _repeats: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        // let publisher = self.publisher_ucs.get_publisher_by_social_network(social_network);
        // Todo: Add publisher
        Ok(())
    }
}
