#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PublisherAuth {
    Telegram { token: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PublisherConfig {
    Telegram {
        chat_id: i64,
        caption: Option<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Publisher {
    pub id: uuid::Uuid,

    pub auth: PublisherAuth,
    pub config: PublisherConfig,
    pub repeats: Vec<String>,

    pub social_network: super::SocialNetwork,
    pub supported_media: Vec<super::MediaType>,

    pub created_at: i64,
}
