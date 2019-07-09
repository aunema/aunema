#[derive(Debug, Deserialize)]
pub struct RedditPosts {
    pub kind: String,
    pub data: RedditData,
}

#[derive(Debug, Deserialize)]
pub struct RedditData {
    pub dist: i64,
    pub children: Vec<RedditChild>,
}

#[derive(Debug, Deserialize)]
pub struct RedditChild {
    pub kind: String,
    pub data: RedditChildData,
}

#[derive(Debug, Deserialize)]
pub struct RedditChildData {
    pub id: String,
    pub subreddit: String,
    pub title: String,
    pub score: i64,
    pub created: f64,
    pub domain: String,
    pub author: String,
    pub url: String,
    pub is_video: bool,
}
