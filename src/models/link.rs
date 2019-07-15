#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: uuid::Uuid,

    pub provider: String,
    pub media_limit: u16,
    pub social_network: super::SocialNetwork,

    pub created_at: i64,
}
