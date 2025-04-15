use crate::posts::client::{GetByIdRequest, PostId};
use crate::state::AppState;
use super::models::{Post, TokenClaims};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use chrono::DateTime;
use jsonwebtoken::{DecodingKey, Validation, decode};
use tonic::Code;

pub async fn post_get(
    jar: CookieJar,
    Path(post_id): Path<i64>,
    State(mut app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let session = jar
        .get("session")
        .map(|cookie| cookie.value().to_owned())
        .unwrap_or(String::new());
    let claims = decode::<TokenClaims>(
        &session,
        &DecodingKey::from_secret(app_state.secret.as_ref()),
        &Validation::default(),
    )
    .or(Err(StatusCode::UNAUTHORIZED))?
    .claims;

    let res = app_state
        .grpc_client
        .get_post_by_id(GetByIdRequest {
            post_id: Some(PostId { post_id }),
            creator_id: claims.userid,
        })
        .await;
    match res {
        Err(status) => match status.code() {
            Code::NotFound => Err(StatusCode::NOT_FOUND),
            Code::PermissionDenied => Err(StatusCode::FORBIDDEN),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(post) => {
            let post = post.into_inner();
            let creation_time = DateTime::from_timestamp(
                post.creation_time.unwrap().seconds,
                post.creation_time.unwrap().nanos as u32,
            )
            .unwrap();
            let update_time = DateTime::from_timestamp(
                post.update_time.unwrap().seconds,
                post.update_time.unwrap().nanos as u32,
            )
            .unwrap();
            let post_resp = Post {
                post_id: post.id.unwrap().post_id,
                name: post.name,
                description: post.description,
                creator_id: post.creator_id,
                creation_time,
                update_time,
                is_private: post.is_private,
                tags: post.tags,
            };
            Ok(Json(post_resp))
        }
    }

    // match res {
    //     Err(_) => Response::builder()
    //         .status(StatusCode::INTERNAL_SERVER_ERROR)
    //         .body(Body::empty())
    //         .unwrap(),
    //     Ok(res) => {
    //         let status = res.status();
    //         if !status.is_success() {
    //             return Response::builder()
    //                 .status(status)
    //                 .body(Body::empty())
    //                 .unwrap();
    //         }
    //         match res.text().await {
    //             Err(_) => Response::builder()
    //                 .status(StatusCode::INTERNAL_SERVER_ERROR)
    //                 .body(Body::empty())
    //                 .unwrap(),
    //             Ok(text) => Response::builder()
    //                 .status(status)
    //                 .body(Body::from(text))
    //                 .unwrap(),
    //         }
    //     }
    // }
}
