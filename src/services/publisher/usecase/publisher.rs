#![allow(dead_code)]

use crate::models::{MediaType, Publisher, SocialNetwork};

use chrono::Utc;
use std::error::Error;

impl super::PublisherUsecase {
    pub fn add_publisher(
        &self,
        chat_id: i64,
        social_network: SocialNetwork,
        supported_media: Vec<MediaType>,
        caption: Option<String>,
        repeats: Vec<String>,
    ) -> Result<Publisher, Box<dyn Error>> {
        let client = self.db_pool.get()?;
        let publisher = Publisher {
            id: uuid::Uuid::new_v4(),
            chat_id,
            social_network,
            supported_media,
            caption,
            repeats,
            created_at: Utc::now().timestamp(),
        };
        let result = client.execute(
            "INSERT INTO publisherss VALUES($1, $2, $3, $4, $5, $6, $7)",
            &[
                &publisher.id,
                &publisher.chat_id,
                &publisher.social_network,
                &publisher.supported_media,
                &publisher.caption,
                &publisher.repeats,
                &publisher.created_at,
            ],
        )?;
        match result {
            1 => Ok(publisher),
            _ => Err(Box::from("Failed to add publisher")),
        }
    }
}
