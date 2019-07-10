use crate::models::{Media, MediaType, SocialNetwork, UseStatus};

use chrono::Utc;
use postgres::types::ToSql;
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

        for row in &client
            .query(
                "
                SELECT
                    id, unique_identifier, duration, used_in,
                    use_status, social_network, media_type, created_at
                FROM media
                WHERE unique_identifier = ANY($1)
            ",
                &[&uids],
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

    pub fn add_media(&self, media: Vec<Media>) -> Result<u64, Box<dyn Error>> {
        let client = self.db_pool.get()?;

        let mut items_count_in_struct = 0;
        let mut media_items = vec![];
        for media_item in media {
            let items = media_item.to_sql_vec();
            items_count_in_struct = items.len();
            media_items.extend(items);
        }
        let values: Vec<&dyn ToSql> = media_items.iter_mut().map(|item| &**item).collect();

        // Todo: Fix or move to template function
        let mut query_values = String::from("VALUES(");
        for (index, _) in values.iter().enumerate() {
            let num = index + 1;
            let sep = num % items_count_in_struct;
            if sep == 0 {
                query_values += &format!("${}), (", num);
            } else {
                query_values += &format!("${}, ", num);
            }
        }
        query_values.truncate(query_values.len() - 3);

        let result = client.execute(
            &format!(
                "INSERT INTO media (
                id, unique_identifier, duration, used_in,
                use_status, social_network, media_type, created_at
            ) {}",
                query_values
            ),
            values.as_slice(),
        )?;
        match result {
            0 => Err(Box::from("Failed to add media")),
            _ => Ok(result),
        }
    }
}
