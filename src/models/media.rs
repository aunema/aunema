#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: uuid::Uuid,
    pub unique_identifier: String,

    pub duration: Option<i64>,
    pub used_in: Option<uuid::Uuid>,

    pub use_status: super::UseStatus,
    pub social_network: super::SocialNetwork,
    pub media_type: super::MediaType,

    pub created_at: i64,
}
