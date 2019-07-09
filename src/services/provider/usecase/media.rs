use crate::models::{Media, MediaType, SocialNetwork, UseStatus};

use chrono::Utc;
use std::error::Error;

impl super::ProviderUsecase {
    pub fn create_media(
        &self,
        uid: String,
        use_status: UseStatus,
        social_network: SocialNetwork,
        media_type: MediaType,
    ) -> Media {
        Media {
            id: uuid::Uuid::new_v4(),
            unique_identifier: uid,
            duration: None,
            used_in: None,
            use_status,
            social_network,
            media_type,
            created_at: Utc::now().timestamp(),
        }
    }

    pub fn get_media_by_uids(&self, uids: Vec<String>) -> Result<Vec<Media>, Box<dyn Error>> {
        let client = self.db_pool.get()?;
        let mut media: Vec<Media> = Vec::new();

        // Todo: Fix these ugly lines
        let mut in_uids = String::from("");
        uids.iter().for_each(|value| {
            if in_uids == "" {
                in_uids += value;
            } else {
                in_uids += &format!(", {}", &value);
            }
        });

        for row in &client
            .query(
                "
                SELECT
                    id, unique_identifier, duration, used_in,
                    use_status, social_network, media_type, created_at
                FROM media
                WHERE unique_identifier IN ($1)
            ",
                &[&in_uids],
            )
            .unwrap()
        {
            let saved_media = Media {
                id: row.get(0),
                unique_identifier: row.get(1),
                duration: row.get(2),
                used_in: row.get(3),
                use_status: row.get(4),
                social_network: row.get(5),
                media_type: row.get(6),
                created_at: row.get(7),
            };
            media.push(saved_media);
        }
        Ok(media)
    }
}
