use axum::{
    Router,
    routing::post,
};
use crate::state::AppState;

// mod login;
// mod user_get;
// mod user_post;
// mod user_put;
mod stats_view;
mod stats_likes;
mod stats_comment;


// use login::login;
// use user_get::user_get;
// use user_post::user_post;
// use user_put::user_put;
use stats_view::stats_view;
use stats_likes::stats_like;
use stats_comment::stats_comment;

pub fn route_stats(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/view", post(stats_view))
        .route("/like", post(stats_like))
        .route("/comment", post(stats_comment))
}
