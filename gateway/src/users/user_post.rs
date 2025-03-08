use axum::{extract::Json, http::StatusCode};
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct User {
    username: String,
    password: String,
    name: String,
    surname: String,
    birthdate: String,
    status: String,
    mail: String,
    phone: String,
}

pub async fn user_post(Json(user): Json<User>) -> StatusCode {
    let client = reqwest::Client::new();
    let res = client
        .post("http://users:3000/user")
        .json(&user)
        .send()
        .await;
    if res.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    let res = res.unwrap();
    res.status()
}
