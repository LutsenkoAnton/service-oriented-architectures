use args::Args;
use axum::Router;
use clap::Parser;
use users::route_users;

mod args;
mod users;

async fn init_app() -> Router {
    let mut router = Router::new();
    router = route_users(router);
    router
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, init_app().await).await.unwrap();
}
