use postgres::types::{IsNull, Kind, ToSql, Type};
use std::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProviderAuth {
    Reddit {},
}

impl postgres::types::FromSql for ProviderAuth {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        serde_json::from_slice(&raw).map_err(|_| Box::from("FATAL: Failed to deserialize data"))
    }

    fn accepts(ty: &Type) -> bool {
        <Self as ToSql>::accepts(ty)
    }
}

impl postgres::types::ToSql for ProviderAuth {
    fn to_sql(&self, _: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        *out = serde_json::to_vec(self)?;
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        match *ty.kind() {
            Kind::Composite(_) => true,
            _ => false,
        }
    }

    to_sql_checked!();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProviderConfig {
    Reddit { limit: i64 },
}

impl postgres::types::FromSql for ProviderConfig {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        serde_json::from_slice(&raw).map_err(|_| Box::from("FATAL: Failed to deserialize data"))
    }

    fn accepts(ty: &Type) -> bool {
        <Self as ToSql>::accepts(ty)
    }
}

impl postgres::types::ToSql for ProviderConfig {
    fn to_sql(&self, _: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        *out = serde_json::to_vec(self)?;
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        match *ty.kind() {
            Kind::Composite(_) => true,
            _ => false,
        }
    }

    to_sql_checked!();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Provider {
    pub id: uuid::Uuid,

    pub auth: ProviderAuth,
    pub config: ProviderConfig,
    pub repeats: Vec<String>,

    pub social_network: super::SocialNetwork,
    pub supported_media: Vec<super::MediaType>,

    pub created_at: i64,
}
