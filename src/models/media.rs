use postgres::types::ToSql;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: uuid::Uuid,
    pub unique_identifier: String,
    pub data_url: Option<String>,

    pub duration: Option<i64>,
    pub used_in: Option<uuid::Uuid>,

    pub use_status: super::UseStatus,
    pub social_network: super::SocialNetwork,
    pub media_type: super::MediaType,

    pub created_at: i64,
}

impl Media {
    pub fn to_sql_vec<'a>(&self) -> Vec<Box<dyn ToSql>> {
        let mut sql_vec: Vec<Box<dyn ToSql>> = vec![];
        sql_vec.push(Box::new(self.id));
        sql_vec.push(Box::new(self.unique_identifier.clone()));
        sql_vec.push(Box::new(self.data_url.clone()));
        sql_vec.push(Box::new(self.duration));
        sql_vec.push(Box::new(self.used_in));
        sql_vec.push(Box::new(self.use_status));
        sql_vec.push(Box::new(self.social_network));
        sql_vec.push(Box::new(self.media_type));
        sql_vec.push(Box::new(self.created_at));
        sql_vec
    }
}
