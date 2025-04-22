use super::client::{PostId, DeletePostRequest};
use super::models::TokenClaims;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, Validation, decode};
use tonic::Code;

pub async fn post_delete(
    jar: CookieJar,
    State(mut app_state): State<AppState>,
    Path(post_id): Path<i64>,
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
    let res = app_state
        .grpc_client
        .delete_post(DeletePostRequest {
            post_id: Some(PostId { post_id }),
            creator_id: claims.userid,
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
