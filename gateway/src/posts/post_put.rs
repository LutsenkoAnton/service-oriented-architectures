use super::client::{PostId, UpdatePostRequest};
use super::models::TokenClaims;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, Validation, decode};
use prost_types::FieldMask;
use tonic::Code;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Post {
    name: Option<String>,
    description: Option<String>,
    is_private: Option<bool>,
    tags: Option<Vec<String>>,
}

pub async fn post_put(
    jar: CookieJar,
    State(mut app_state): State<AppState>,
    Path(post_id): Path<i64>,
    Json(post): Json<Post>,
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
    let mut field_mask = Vec::<String>::new();
    if post.name.is_some() {
        field_mask.push("name".to_string());
    }
    if post.description.is_some() {
        field_mask.push("description".to_string());
    }
    if post.is_private.is_some() {
        field_mask.push("is_private".to_string());
    }
    if post.tags.is_some() {
        field_mask.push("tags".to_string());
    }

    let res = app_state
        .grpc_client
        .update_post(UpdatePostRequest {
            post_id: Some(PostId { post_id }),
            name: post.name,
            description: post.description,
            creator_id: claims.userid,
            is_private: post.is_private,
            tags: post.tags.unwrap_or_default(),
            field_mask: Some(FieldMask { paths: field_mask }),
        })
        .await;
    match res {
        Err(status) => match status.code() {
            Code::NotFound => StatusCode::NOT_FOUND,
            Code::PermissionDenied => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Ok(_) => StatusCode::OK,
    }
}
