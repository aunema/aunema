#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProviderAuth {
    Reddit {},
}

impl postgres::types::FromSql for ProviderAuth {
    fn from_sql(
        _: &postgres::types::Type,
        raw: &[u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        serde_json::from_slice(&raw).map_err(|_| Box::from("FATAL: Failed to deserialize data"))
    }

    fn accepts(ty: &postgres::types::Type) -> bool {
        <serde_json::Value as postgres::types::FromSql>::accepts(ty)
    }
}

impl postgres::types::ToSql for ProviderAuth {
    fn to_sql(
        &self,
        _: &postgres::types::Type,
        out: &mut Vec<u8>,
    ) -> Result<postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        *out = serde_json::to_vec(self)?;
        Ok(postgres::types::IsNull::No)
    }

    fn accepts(ty: &postgres::types::Type) -> bool {
        serde_json::Value::accepts(ty)
    }

    to_sql_checked!();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProviderConfig {
    Reddit { limit: i64 },
}

impl postgres::types::FromSql for ProviderConfig {
    fn from_sql(
        _: &postgres::types::Type,
        raw: &[u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        serde_json::from_slice(&raw).map_err(|_| Box::from("FATAL: Failed to deserialize data"))
    }

    fn accepts(ty: &postgres::types::Type) -> bool {
        <serde_json::Value as postgres::types::FromSql>::accepts(ty)
    }
}

impl postgres::types::ToSql for ProviderConfig {
    fn to_sql(
        &self,
        _: &postgres::types::Type,
        out: &mut Vec<u8>,
    ) -> Result<postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        *out = serde_json::to_vec(self)?;
        Ok(postgres::types::IsNull::No)
    }

    fn accepts(ty: &postgres::types::Type) -> bool {
        serde_json::Value::accepts(ty)
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
