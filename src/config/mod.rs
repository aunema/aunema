use envy::{from_env, prefixed};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub production: bool,
    pub server_port: i16,
    pub db_connection: String,

    #[serde(skip_deserializing)]
    pub storage: Option<Storage>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Storage {
    pub auth: String,
    pub temporary: String,
    pub finished: String,
    pub elements: String,
    pub production: String,
}

impl Config {
    pub fn init() -> Result<Arc<Self>, Box<std::error::Error>> {
        let mut cnfg = from_env::<Config>()?;
        let storage = prefixed("STORAGE_").from_env::<Storage>()?;
        cnfg.storage = Some(storage);
        Ok(Arc::new(cnfg))
    }
}
