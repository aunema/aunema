use crate::models::{Link, SocialNetwork};

use std::error::Error;

impl super::ProviderController {
    pub fn add_link(
        &self,
        data: String,
        social_network: SocialNetwork,
    ) -> Result<Link, Box<dyn Error>> {
        match self.provider_ucs.get_link_by_data(data.clone())? {
            Some(_) => return Err(Box::from("Link already exists")),
            None => (),
        };
        self.provider_ucs.add_link(data, social_network)
    }

    pub fn get_links(
        &self,
        social_network: SocialNetwork,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Link>, Box<dyn Error>> {
        self.provider_ucs.get_links(social_network, limit, offset)
    }
}
