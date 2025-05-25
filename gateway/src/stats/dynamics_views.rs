use crate::models::TokenClaims;
use crate::state::AppState;
use crate::stats::client::{ActivityType, DynamicsRequest};
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
struct Dynamic {
    count: u64,
    day: chrono::NaiveDate,
}

fn convert_grpcstatus_to_httpstatus(status: Status) -> StatusCode {
    match status.code() {
        Code::NotFound => StatusCode::NOT_FOUND,
        Code::PermissionDenied => StatusCode::FORBIDDEN,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn dynamics_views(
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
        .dynamics(DynamicsRequest {
            post_id,
            r#type: ActivityType::Views.into(),
        })
        .await;
    match res {
        Err(status) => Err(convert_grpcstatus_to_httpstatus(status)),
        Ok(dynamics) => {
            let mut response = Vec::new();
            let mut stream = dynamics.into_inner();

            while let Some(dynamic) = stream.message().await.map_err(convert_grpcstatus_to_httpstatus)? {
                dbg!(&dynamic);
                response.push(Dynamic {
                    count: dynamic.count,
                    day: dynamic.day.parse().unwrap(),
                })
            }
            Ok(Json(response))
        }
    }
}
