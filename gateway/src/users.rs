use axum::{
    Router,
    routing::{get, post, put},
};

mod login;
mod user_get;
mod user_post;
mod user_put;

use login::login;
use user_get::user_get;
use user_post::user_post;
use user_put::user_put;

pub fn route_users(router: Router) -> Router {
    router
        .route("/login", get(login))
        .route("/user", post(user_post))
        .route("/user/{username}", get(user_get))
        .route("/user/{username}", put(user_put))
}
