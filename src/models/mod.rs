#![allow(dead_code)]

mod media;
mod link;

pub use media::Media;
pub use link::Link;

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UseStatus {
    Used = 0,
    Normal = 1,
    Priority = 2,
    Urgent = 3,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum SocialNetwork {
    Reddit = 0,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MediaType {
    Image = 0,
    Animated = 1,
    Video = 2,
}
