use axum::{
    Router,
    routing::{post, get},
};
use crate::state::AppState;

pub mod client;
mod get_stats;
mod dynamics_comments;
mod dynamics_likes;
mod dynamics_views;
mod top_posts;
mod top_users;

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
use get_stats::get_stats;
use dynamics_comments::dynamics_comments;
use dynamics_likes::dynamics_likes;
use dynamics_views::dynamics_views;
use top_posts::top_posts;
use top_users::top_users;
use stats_view::stats_view;
use stats_likes::stats_like;
use stats_comment::stats_comment;

pub fn route_stats(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/view", post(stats_view))
        .route("/like", post(stats_like))
        .route("/comment", post(stats_comment))
        .route("/stats/{post_id}", get(get_stats))
        .route("/dynamics/comments/{post_id}", get(dynamics_comments))
        .route("/dynamics/likes/{post_id}", get(dynamics_likes))
        .route("/dynamics/views/{post_id}", get(dynamics_views))
        .route("/top/posts/{order}", get(top_posts))
        .route("/top/users/{order}", get(top_users))
}
