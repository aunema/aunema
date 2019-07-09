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
                SELECT id, data, social_network, created_at
                FROM links
                WHERE social_network = $1
                ORDER BY created_at
                LIMIT $2 OFFSET $3
            ",
                &[&(social_network as i64), &(limit as i64), &(offset as i64)],
            )
            .unwrap()
        {
            let link = Link {
                id: row.get(0),
                data: row.get(1),
                social_network: row.get(2),
                created_at: row.get(3),
            };
            links.push(link);
        }
        Ok(links)
    }

    pub fn add_link(
        &self,
        data: String,
        social_network: SocialNetwork,
    ) -> Result<Link, Box<dyn Error>> {
        let client = self.db_pool.get()?;
        let link = Link {
            id: uuid::Uuid::new_v4(),
            data,
            social_network,
            created_at: Utc::now().timestamp(),
        };
        let result = client.execute(
            "INSERT INTO links VALUES($1, $2, $3, $4)",
            &[
                &link.id,
                &link.data,
                &(link.social_network as i64),
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

    pub fn get_link_by_data(&self, data: String) -> Result<Option<Link>, Box<dyn Error>> {
        let client = self.db_pool.get()?;
        for row in &client
            .query(
                "SELECT id, data, social_network, created_at FROM links WHERE data = $1",
                &[&data],
            )
            .unwrap()
        {
            let link = Link {
                id: row.get(0),
                data: row.get(1),
                social_network: row.get(2),
                created_at: row.get(3),
            };
            return Ok(Some(link));
        }
        Ok(None)
    }
}
