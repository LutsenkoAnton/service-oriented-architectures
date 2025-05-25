use crate::models::TokenClaims;
use crate::state::AppState;
use crate::stats::client::{ActivityType, GetTop10PostsRequest};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, Validation, decode};
use tonic::{Status, Code};

#[derive(Debug, Clone, serde::Serialize)]
struct Post {
    post_id: i64,
}

fn convert_grpcstatus_to_httpstatus(status: Status) -> StatusCode {
    match status.code() {
        Code::NotFound => StatusCode::NOT_FOUND,
        Code::PermissionDenied => StatusCode::FORBIDDEN,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[axum::debug_handler]
pub async fn top_posts(
    jar: CookieJar,
    Path(order): Path<ActivityType>,
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
        .get_top10_posts(GetTop10PostsRequest {
            r#type: order.into(),
        })
        .await;
    match res {
        Err(status) => Err(convert_grpcstatus_to_httpstatus(status)),
        Ok(dynamics) => {
            let mut response = Vec::new();
            let mut stream = dynamics.into_inner();

            while let Some(post) = stream.message().await.map_err(convert_grpcstatus_to_httpstatus)? {
                response.push(Post {
                    post_id: post.post_id,
                })
            }
            Ok(Json(response))
        }
    }
}
