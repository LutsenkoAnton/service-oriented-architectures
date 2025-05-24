use crate::models::TokenClaims;
use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use axum_extra::extract::cookie::CookieJar;
use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, Validation, decode};
use rdkafka::producer::FutureRecord;
use std::time::Duration;
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Comment {
    post_id: i64,
    comment: String
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CommentKafka {
    post_id: i64,
    creator_id: i32,
    comment: String,
    time: DateTime<Utc>,
}

pub async fn stats_comment(
    jar: CookieJar,
    State(app_state): State<AppState>,
    Json(comment): Json<Comment>,
) -> StatusCode {
    let session = jar
        .get("session")
        .map(|cookie| cookie.value().to_owned())
        .unwrap_or(String::new());
    let claims = decode::<TokenClaims>(
        &session,
        &DecodingKey::from_secret(app_state.secret.as_ref()),
        &Validation::default(),
    );
    if claims.is_err() {
        return StatusCode::UNAUTHORIZED;
    }
    let claims = claims.unwrap().claims;
    let view = CommentKafka {
        post_id: comment.post_id,
        creator_id: claims.userid,
        comment: comment.comment,
        time: Utc::now(),
    };
    let str_view = serde_json::to_string(&view).unwrap();
    let produce_future = app_state.kafka_stats.send(
        FutureRecord::<std::string::String, std::string::String>::to("comments").payload(&str_view),
        Duration::from_secs(0),
    );
    match produce_future.await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
