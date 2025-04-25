use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use rdkafka::producer::FutureRecord;
use std::time::Duration;

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

#[derive(Debug, Clone, serde::Deserialize)]
struct UserId {
    id: i32,
}

#[derive(Debug, Clone, serde::Serialize)]
struct KafkaUser {
    id: i32,
    time: DateTime<Utc>,
}

pub async fn user_post(State(app_state): State<AppState>, Json(user): Json<User>) -> StatusCode {
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
    let status = res.status();
    let id = res.json::<UserId>().await.unwrap().id;
    let str_user = serde_json::to_string(&KafkaUser {
        id,
        time: Utc::now(),
    })
    .unwrap();
    let _ = app_state
        .kafka_stats
        .send(
            FutureRecord::<std::string::String, std::string::String>::to("registrations")
                .payload(&str_user),
            Duration::from_secs(0),
        )
        .await;
    status
}
