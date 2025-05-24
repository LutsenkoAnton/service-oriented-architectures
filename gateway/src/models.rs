use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: u64,
    pub username: String,
    pub userid: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub post_id: i64,
    pub name: String,
    pub description: String,
    pub creator_id: i32,
    pub creation_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub is_private: bool,
    pub tags: Vec<String>,
}

