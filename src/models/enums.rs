use postgres::types::{IsNull, ToSql, Type};
use std::error::Error;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Copy, Clone, Debug, Primitive, Deserialize_repr, Serialize_repr, EnumSql)]
#[repr(i64)]
pub enum UseStatus {
    Used = 0,
    Normal = 1,
    Priority = 2,
    Urgent = 3,
}

#[derive(Copy, Clone, Debug, Primitive, Deserialize_repr, Serialize_repr, EnumSql)]
#[repr(i64)]
pub enum SocialNetwork {
    Reddit = 0,
    Telegram = 1,
}

#[derive(Copy, Clone, Debug, Primitive, Deserialize_repr, Serialize_repr, EnumSql)]
#[repr(i64)]
pub enum MediaType {
    Image = 0,
    Animated = 1,
    Video = 2,
}
