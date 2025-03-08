use crate::{Result, initialize::AppState, model::Username};
use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Default, Clone, sqlx::FromRow, serde::Serialize)]
struct User {
    name: String,
    surname: String,
    birthdate: String,
    status: String,
    mail: String,
    phone: String,
}

async fn inner_get_user(username: String, state: AppState) -> Result<Response> {
    let result = sqlx::query_as!(
        User,
        "SELECT name, surname, birthdate, status, mail, phone FROM users WHERE username=$1;",
        username
    )
    .fetch_optional(&state.pool)
    .await?;
    if let Some(user) = result {
        Ok(Json(user).into_response())
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap())
    }
}

pub async fn get_user(State(state): State<AppState>, Path(path): Path<Username>) -> Response {
    inner_get_user(path.username, state)
        .await
        .expect("get_user failed")
}
