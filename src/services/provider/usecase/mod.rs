mod media;
mod links;

use crate::config::Config;
use crate::helpers::database::Database;
use crate::models::{SocialNetwork, Provider};

use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProviderUsecase {
    pub cnfg: Arc<Config>,
    pub db_pool: Database,
}

pub fn init(cnfg: &Arc<Config>, db_pool: &Database) -> Arc<ProviderUsecase> {
    let cnr = ProviderUsecase {
        cnfg: cnfg.clone(),
        db_pool: db_pool.clone(),
    };
    Arc::new(cnr)
}

impl ProviderUsecase {
    pub fn get_provider(&self, social_network: SocialNetwork) -> Result<Option<Provider>, Box<dyn Error>> {
        let client = self.db_pool.get()?;
        for row in &client
            .query(
                "SELECT id, auth, config, repeats, social_network, supported_media, created_at
                FROM providers WHERE social_network = $1",
                &[&social_network],
            )
            .unwrap()
        {
            let provider = Provider {
                id: row.get(0),
                auth: row.get(1),
                config: row.get(2),
                repeats: row.get(3),
                social_network: row.get(4),
                supported_media: row.get(5),
                created_at: row.get(6),
            };
            return Ok(Some(provider));
        }
        Ok(None)
    }
}
