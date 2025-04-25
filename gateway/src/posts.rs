use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post, put, delete},
};

pub mod client;
mod post_get;
mod post_get_page;
mod post_post;
mod post_put;
mod post_delete;

use post_get::post_get;
use post_get_page::post_get_page;
use post_post::post_post;
use post_put::post_put;
use post_delete::post_delete;

pub fn route_posts(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/post/{post_id}", get(post_get))
        .route("/post/page/{from}/{limit}", get(post_get_page))
        .route("/post/{post_id}", put(post_put))
        .route("/post/{post_id}", delete(post_delete))
        .route("/post", post(post_post))
    // .route("/user/{username}", get(user_get))
    // .route("/user/{username}", put(user_put))
}
