mod publisher;

use crate::config::Config;
use crate::helpers::database::Database;

use std::sync::Arc;

#[derive(Clone)]
pub struct PublisherUsecase {
    pub cnfg: Arc<Config>,
    pub db_pool: Database,
}

pub fn init(cnfg: &Arc<Config>, db_pool: &Database) -> Arc<PublisherUsecase> {
    let cnr = PublisherUsecase {
        cnfg: cnfg.clone(),
        db_pool: db_pool.clone(),
    };
    Arc::new(cnr)
}
