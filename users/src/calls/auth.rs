use crate::initialize::AppState;
use crate::model::Username;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: u64,
    pub username: String,
    pub userid: i32,
}

pub async fn encode_jwt(username: &str, userid: i32, state: &AppState) -> Result<String, StatusCode> {
    let claims = TokenClaims {
        exp: (Utc::now() + Duration::days(1)).timestamp() as u64,
        username: username.to_string(),
        userid
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.secret.as_ref()),
    )
    .or(Err(StatusCode::BAD_REQUEST))
}

pub async fn auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let token = jar.get("session").map(|cookie| cookie.value().to_string());
    if token.is_none() {
        return Err(StatusCode::FORBIDDEN);
    }

    let token = token.unwrap();

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.secret.as_ref()),
        &Validation::default(),
    )
    .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
    .claims;

    let username = claims.username;

    req.extensions_mut().insert(Username { username });
    Ok(next.run(req).await)
}
