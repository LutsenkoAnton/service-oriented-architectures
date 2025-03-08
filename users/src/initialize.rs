use crate::{Result, calls::*};
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post, put},
};
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub secret: String,
}

pub async fn init_db() -> Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("database url env variable shoudl be set"))
        .await?)
}

pub async fn init_app() -> Router {
    let secret = std::env::var("SECRET").expect("SECRET should be set");
    let app_state = AppState {
        pool: init_db().await.unwrap(),
        secret,
    };
    Router::new()
        .route("/user", post(create_user))
        .route(
            "/user/{username}",
            get(get_user).layer(from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/user/{username}",
            put(update_user).layer(from_fn_with_state(app_state.clone(), auth)),
        )
        .route("/login", get(login))
        .with_state(app_state)
}
