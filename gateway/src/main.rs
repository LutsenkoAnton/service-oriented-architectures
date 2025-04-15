use args::Args;
use axum::Router;
use clap::Parser;
use users::route_users;
use state::AppState;
use posts::{route_posts, client::PostsServerClient};
use std::sync::{Arc, Mutex};

mod args;
mod users;
mod state;
mod posts;

async fn init_app() -> Router {
    let app_state = AppState {
        grpc_client: PostsServerClient::connect("http://posts:5000").await.unwrap(),
        secret: std::env::var("SECRET").expect("SECRET should be set"),
    };
    let mut router:Router<AppState> = Router::new();
    router = route_users(router);
    router = route_posts(router);
    router.with_state(app_state)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, init_app().await).await.unwrap();
}
