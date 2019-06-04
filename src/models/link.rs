#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: uuid::Uuid,

    pub data: String,
    pub social_network: super::SocialNetwork,

    pub created_at: i64,
}
