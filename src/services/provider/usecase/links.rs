use crate::models::{Link, SocialNetwork};

use chrono::Utc;
use std::error::Error;

impl super::ProviderUsecase {
    pub fn get_links(
        &self,
        social_network: SocialNetwork,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Link>, Box<dyn Error>> {
        let client = self.db_pool.get()?;
        let mut links: Vec<Link> = Vec::new();
        for row in &client
            .query(
                "
                SELECT id, provider, media_limit, social_network, created_at
                FROM links
                WHERE social_network = $1
                ORDER BY created_at
                LIMIT $2 OFFSET $3
            ",
                &[&(social_network as i64), &(limit as i64), &(offset as i64)],
            )
            .unwrap()
        {
            let media_limit: i64 = row.get(2);
            let link = Link {
                id: row.get(0),
                provider: row.get(1),
                media_limit: media_limit as u16,
                social_network: row.get(3),
                created_at: row.get(4),
            };
            links.push(link);
        }
        Ok(links)
    }

    pub fn add_link(
        &self,
        provider: String,
        media_limit: u16,
        social_network: SocialNetwork,
    ) -> Result<Link, Box<dyn Error>> {
        let client = self.db_pool.get()?;
        let link = Link {
            id: uuid::Uuid::new_v4(),
            provider,
            media_limit,
            social_network,
            created_at: Utc::now().timestamp(),
        };
        let result = client.execute(
            "INSERT INTO links VALUES($1, $2, $3, $4, $5)",
            &[
                &link.id,
                &link.provider,
                &(link.media_limit as i64),
                &link.social_network,
                &link.created_at,
            ],
        )?;
        match result {
            1 => Ok(link),
            _ => Err(Box::from("Failed to add link")),
        }
    }

    pub fn remove_link(&self, id: uuid::Uuid) -> Result<(), Box<dyn Error>> {
        let client = self.db_pool.get()?;
        let result = client.execute("DELETE FROM links WHERE id = $1", &[&id])?;
        match result {
            1 => Ok(()),
            _ => Err(Box::from("Link not found")),
        }
    }

    pub fn get_link_by_provider(&self, provider: String) -> Result<Option<Link>, Box<dyn Error>> {
        let client = self.db_pool.get()?;
        for row in &client
            .query(
                "SELECT id, provider, media_limit, social_network, created_at
                FROM links WHERE provider = $1",
                &[&provider],
            )
            .unwrap()
        {
            let media_limit: i64 = row.get(2);
            let link = Link {
                id: row.get(0),
                provider: row.get(1),
                media_limit: media_limit as u16,
                social_network: row.get(3),
                created_at: row.get(4),
            };
            return Ok(Some(link));
        }
        Ok(None)
    }
}
