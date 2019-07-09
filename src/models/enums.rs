use postgres::types::{FromSql, ToSql, Type};
use postgres_protocol::types;
use std::error::Error;

#[derive(Copy, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UseStatus {
    Used = 0,
    Normal = 1,
    Priority = 2,
    Urgent = 3,
}

// Todo: Create derive macro
impl FromSql for UseStatus {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let value = types::int8_from_sql(raw)?;
        let result: Self = serde_json::from_str(&value.to_string())?;
        Ok(result)
    }

    fn accepts(ty: &Type) -> bool {
        <i64 as ToSql>::accepts(ty)
    }
}

#[derive(Copy, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum SocialNetwork {
    Reddit = 0,
}

// Todo: Create derive macro
impl FromSql for SocialNetwork {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let value = types::int8_from_sql(raw)?;
        let result: Self = serde_json::from_str(&value.to_string())?;
        Ok(result)
    }

    fn accepts(ty: &Type) -> bool {
        <i64 as ToSql>::accepts(ty)
    }
}

#[derive(Copy, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MediaType {
    Image = 0,
    Animated = 1,
    Video = 2,
}

// Todo: Create derive macro
impl FromSql for MediaType {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let value = types::int8_from_sql(raw)?;
        let result: Self = serde_json::from_str(&value.to_string())?;
        Ok(result)
    }

    fn accepts(ty: &Type) -> bool {
        <i64 as ToSql>::accepts(ty)
    }
}
