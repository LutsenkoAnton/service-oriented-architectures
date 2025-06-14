use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub name: String,
    pub surname: String,
    pub birthdate: String,
    pub status: String,
    pub mail: String,
    pub phone: String,
}


#[derive(Debug, Serialize)]
pub struct Post {
    pub name: String,
    pub description: String,
    pub is_private: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostId {
    pub post_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Acitivity {
    pub comments: u64,
    pub likes: u64,
    pub views: u64,
}

#[derive(Debug, Serialize)]
pub struct ChangePrivacy {
    pub is_private: bool,
}
