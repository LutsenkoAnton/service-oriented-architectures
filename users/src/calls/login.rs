use super::auth::encode_jwt;
use crate::initialize::AppState;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
}

pub async fn login(
    jar: CookieJar,
    State(state): State<AppState>,
    Query(params): Query<LoginParams>,
) -> Result<CookieJar, StatusCode> {
    let result = sqlx::query!(
        "SELECT passhash FROM users WHERE username=$1;",
        params.username
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if result.is_none() {
        return Err(StatusCode::FORBIDDEN);
    }
    let pass_hash = PasswordHash::new(&result.as_ref().unwrap().passhash)
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    if Argon2::default()
        .verify_password(&params.password.as_bytes(), &pass_hash)
        .is_ok()
    {
        Ok(jar.add(Cookie::new(
            "session",
            encode_jwt(&params.username, &state).await?,
        )))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
