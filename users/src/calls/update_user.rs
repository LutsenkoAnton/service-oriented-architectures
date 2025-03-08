use crate::{
    Result,
    initialize::AppState,
    model::{Username, check_birthdate, check_email, check_name, check_phone},
};
use axum::{
    Json,
    body::Body,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct User {
    name: Option<String>,
    surname: Option<String>,
    birthdate: Option<String>,
    status: Option<String>,
    mail: Option<String>,
    phone: Option<String>,
}

async fn inner_update_user(username: String, user: User, state: AppState) -> Result<Response> {
    if user.name.is_some() {
        if !check_name(user.name.as_ref().unwrap()) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .unwrap());
        }
    }
    if user.surname.is_some() {
        if !check_name(user.surname.as_ref().unwrap()) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .unwrap());
        }
    }
    if user.birthdate.is_some() {
        if !check_birthdate(user.birthdate.as_ref().unwrap()) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .unwrap());
        }
    }
    if user.mail.is_some() {
        if !check_email(user.mail.as_ref().unwrap()) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .unwrap());
        }
    }
    if user.phone.is_some() {
        if !check_phone(user.phone.as_ref().unwrap()) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .unwrap());
        }
    }
    if user.name.is_some() {
        sqlx::query!(
            "UPDATE users SET name=$1, updated_at=now() WHERE username=$2",
            user.name.unwrap(),
            username
        )
        .execute(&state.pool)
        .await?;
    }
    if user.surname.is_some() {
        sqlx::query!(
            "UPDATE users SET surname=$1, updated_at=now() WHERE username=$2",
            user.surname.unwrap(),
            username
        )
        .execute(&state.pool)
        .await?;
    }
    if user.birthdate.is_some() {
        sqlx::query!(
            "UPDATE users SET birthdate=$1, updated_at=now() WHERE username=$2",
            user.birthdate.unwrap(),
            username
        )
        .execute(&state.pool)
        .await?;
    }
    if user.status.is_some() {
        sqlx::query!(
            "UPDATE users SET status=$1, updated_at=now() WHERE username=$2",
            user.status.unwrap(),
            username
        )
        .execute(&state.pool)
        .await?;
    }
    if user.mail.is_some() {
        sqlx::query!(
            "UPDATE users SET mail=$1, updated_at=now() WHERE username=$2",
            user.mail.unwrap(),
            username
        )
        .execute(&state.pool)
        .await?;
    }
    if user.phone.is_some() {
        sqlx::query!(
            "UPDATE users SET phone=$1, updated_at=now() WHERE username=$2",
            user.phone.unwrap(),
            username
        )
        .execute(&state.pool)
        .await?;
    }
    let result = sqlx::query_as!(
        crate::model::User,
        "SELECT name, surname, birthdate, status, mail, phone FROM users WHERE username=$1;",
        username
    )
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(result).into_response())
}

pub async fn update_user(
    Path(path): Path<Username>,
    Extension(logged_in): Extension<Username>,
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Response {
    if logged_in.username != path.username {
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::empty())
            .unwrap();
    }
    inner_update_user(path.username, user, state)
        .await
        .expect("get_user failed")
}
