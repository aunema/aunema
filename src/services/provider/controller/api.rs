use crate::models::{api::RedditPosts, Media, MediaType, SocialNetwork, UseStatus};

use std::error::Error;

impl super::ProviderController {
    pub fn fetch_data(&self, social_network: SocialNetwork) -> Result<Vec<Media>, Box<dyn Error>> {
        let links = self.provider_ucs.get_links(SocialNetwork::Reddit, 5, 0)?;
        if links.len() <= 0 {
            return Err(Box::from("No links found"));
        }

        let mut result = vec![];
        for link in links {
            let res: RedditPosts = reqwest::get(&link.data)?.json()?;

            for post in res.data.children {
                let data = post.data;

                // Todo: Add support of all media types
                if !data.is_video && data.domain == "i.redd.it" {
                    let new_media = self.provider_ucs.create_media(
                        data.id,
                        UseStatus::Normal,
                        social_network,
                        MediaType::Image,
                    );
                    result.push(new_media);
                }
            }
        }

        Ok(result)
    }
}
