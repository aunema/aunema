use crate::models::{Link, SocialNetwork};

use std::error::Error;
use chrono::Utc;

impl super::ProviderUsecase {
    pub fn get_links(&self, social_network: SocialNetwork, limit: u32, offset: u32) -> Result<Vec<Link>, Box<Error>> {
        let client = self.db_pool.get()?;
        let mut links: Vec<Link> = Vec::new();
        for row in &client
            .query("
                SELECT id, data, social_network, created_at
                FROM links
                WHERE social_network = $1
                ORDER BY created_at
                LIMIT $2 OFFSET $3
            ", &[&(social_network.clone() as i64), &(limit as i64), &(offset as i64)],
            )
            .unwrap()
        {
            let sn_int: i64 = row.get(2);
            let sn: SocialNetwork = match serde_json::from_str(&sn_int.to_string()) {
                Ok(val) => val,
                Err(err) => return Err(err.into()),
            };
            let link = Link {
                id: row.get(0),
                data: row.get(1),
                social_network: sn,
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
    ) -> Result<Link, Box<Error>> {
        let client = self.db_pool.get()?;
        let link = Link {
            id: uuid::Uuid::new_v4(),
            data,
            social_network,
            created_at: Utc::now().timestamp(),
        };
        client.execute(
            "INSERT INTO links VALUES($1, $2, $3, $4)",
            &[
                &link.id,
                &link.data,
                &(link.social_network.clone() as i64),
                &link.created_at,
            ],
        )?;
        Ok(link)
    }

    pub fn get_link_by_data(
        &self,
        data: String,
    ) -> Result<Option<Link>, Box<Error>> {
        let client = self.db_pool.get()?;
        for row in &client
            .query(
                "SELECT id, data, social_network, created_at FROM links WHERE data = $1",
                &[&data],
            )
            .unwrap()
        {
            let sn_int: i64 = row.get(2);
            let sn: SocialNetwork = match serde_json::from_str(&sn_int.to_string()) {
                Ok(val) => val,
                Err(err) => return Err(err.into()),
            };
            let link = Link {
                id: row.get(0),
                data: row.get(1),
                social_network: sn,
                created_at: row.get(3),
            };
            return Ok(Some(link));
        }
        Ok(None)
    }
}
