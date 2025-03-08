use crate::{
    Error, Result,
    initialize::AppState,
    model::{check_birthdate, check_email, check_name, check_phone},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
#[derive(Debug, Clone, serde::Deserialize)]
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

async fn inner_create_user(user: User, state: AppState) -> Result<StatusCode> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(&user.password.clone().into_bytes(), &salt)
        .map_err(|err| Error::Argon2(err))?
        .to_string();
    if !check_name(&user.name) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    if !check_name(&user.surname) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    if !check_birthdate(&user.birthdate) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    if !check_email(&user.mail) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    if !check_phone(&user.phone) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    let row_count = sqlx::query!(
        "INSERT INTO users (username, passhash, name, surname, birthdate, status, mail, phone, updated_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, now(), now()) ON CONFLICT DO NOTHING;",
        user.username,
        password_hash,
        user.name,
        user.surname,
        user.birthdate,
        user.status,
        user.mail,
        user.phone,
    ).execute(&state.pool).await?.rows_affected();
    if row_count == 0 {
        Ok(StatusCode::FORBIDDEN)
    } else {
        Ok(StatusCode::OK)
    }
}
pub async fn create_user(State(state): State<AppState>, Json(user): Json<User>) -> StatusCode {
    inner_create_user(user, state)
        .await
        .expect("create_user failed")
}
