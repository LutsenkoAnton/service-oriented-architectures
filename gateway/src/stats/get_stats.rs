use crate::models::TokenClaims;
use crate::state::AppState;
use crate::stats::client::CountActivityRequest;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, Validation, decode};
use tonic::Code;

#[derive(Debug, Clone, serde::Serialize)]
struct Activity {
    comments: u64,
    likes: u64,
    views: u64,
}

pub async fn get_stats(
    jar: CookieJar,
    Path(post_id): Path<i64>,
    State(mut app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let session = jar
        .get("session")
        .map(|cookie| cookie.value().to_owned())
        .unwrap_or(String::new());
    let _ = decode::<TokenClaims>(
        &session,
        &DecodingKey::from_secret(app_state.secret.as_ref()),
        &Validation::default(),
    )
    .or(Err(StatusCode::UNAUTHORIZED))?
    .claims;

    let res = app_state
        .grpc_client_stats
        .count_activity(CountActivityRequest { post_id })
        .await;
    match res {
        Err(status) => match status.code() {
            Code::NotFound => Err(StatusCode::NOT_FOUND),
            Code::PermissionDenied => Err(StatusCode::FORBIDDEN),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(activity) => {
            let activity = activity.into_inner();
            let activity_resp = Activity {
                comments: activity.comments,
                likes: activity.likes,
                views: activity.views,
            };
            Ok(Json(activity_resp))
        }
    }
}
