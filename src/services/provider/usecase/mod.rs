mod media;
mod links;

use crate::config::Config;
use crate::helpers::database::Database;

use std::sync::Arc;

#[derive(Clone, Debug)]
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

impl ProviderUsecase {}
