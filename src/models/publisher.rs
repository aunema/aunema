// Todo: Make it generic

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Publisher {
    pub id: uuid::Uuid,
    pub chat_id: i64,

    pub social_network: super::SocialNetwork,
    pub supported_media: Vec<super::MediaType>,

    pub caption: Option<String>,
    pub repeats: Vec<String>,

    pub created_at: i64,
}
