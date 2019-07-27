#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PublisherAuth {
    Reddit {},
    Telegram {},
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PublisherConfig {
    Reddit {},
    Telegram {
        chat_id: i64,
        caption: Option<String>,
        repeats: Vec<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Publisher {
    pub id: uuid::Uuid,

    pub social_network: super::SocialNetwork,
    pub supported_media: Vec<super::MediaType>,

    pub auth: PublisherAuth,
    pub config: PublisherConfig,

    pub created_at: i64,
}
