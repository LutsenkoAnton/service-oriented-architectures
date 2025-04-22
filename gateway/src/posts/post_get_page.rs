use crate::posts::client::GetPostsPageRequest;
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

pub async fn post_get_page(
    jar: CookieJar,
    Path((from, limit)): Path<(i32, i32)>,
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
        .get_posts_page(GetPostsPageRequest {
            from,
            limit,
            creator_id: claims.userid,
        })
        .await;
    match res {
        Err(status) => match status.code() {
            Code::NotFound => Err(StatusCode::NOT_FOUND),
            Code::PermissionDenied => Err(StatusCode::FORBIDDEN),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(posts) => {
            let posts = posts.into_inner().posts;
            let response: Vec<Post> = posts
                .iter()
                .map(|post| {
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
                    Post {
                        post_id: post.id.unwrap().post_id,
                        name: post.name.to_owned(),
                        description: post.description.to_owned(),
                        creator_id: post.creator_id,
                        creation_time,
                        update_time,
                        is_private: post.is_private,
                        tags: post.tags.to_owned(),
                    }
                })
                .collect();
            Ok(Json(response))
        }
    }
}
