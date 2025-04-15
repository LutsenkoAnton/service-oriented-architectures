use super::client::CreatePostRequest;
use super::models::TokenClaims;
use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, Validation, decode};
use tonic::Code;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Post {
    name: String,
    description: String,
    is_private: bool,
    tags: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PostId {
    post_id: i64,
}

pub async fn post_post(
    jar: CookieJar,
    State(mut app_state): State<AppState>,
    Json(post): Json<Post>,
) -> Result<impl IntoResponse, StatusCode> {
    let session = jar
        .get("session")
        .map(|cookie| cookie.value().to_owned())
        .unwrap_or(String::new());
    let claims = decode::<TokenClaims>(
        &session,
        &DecodingKey::from_secret(app_state.secret.as_ref()),
        &Validation::default(),
    ).or(Err(StatusCode::UNAUTHORIZED))?.claims;

    let res = app_state
        .grpc_client
        .create_post(CreatePostRequest {
            name: post.name,
            description: post.description,
            creator_id: claims.userid,
            is_private: post.is_private,
            tags: post.tags,
        })
        .await;
    match res {
        Err(status) => match status.code() {
            Code::NotFound => Err(StatusCode::NOT_FOUND),
            Code::PermissionDenied => Err(StatusCode::FORBIDDEN),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(post) => Ok(Json(PostId {
            post_id: post.into_inner().post_id,
        })),
    }
}
