use args::Args;
use axum::Router;
use clap::Parser;
use users::route_users;
use state::AppState;
use posts::{route_posts, client::PostsServerClient};
use stats::{route_stats, client::StatsClient};
use rdkafka::config::ClientConfig;

mod args;
mod users;
mod state;
mod posts;
mod stats;
pub mod models;

async fn init_app() -> Router {
    let brokers = std::env::var("BROKERS").expect("BROKERS should be set");
    let app_state = AppState {
        grpc_client_posts: PostsServerClient::connect("http://posts:5000").await.unwrap(),
        grpc_client_stats: StatsClient::connect("http://stats:7000").await.unwrap(),
        secret: std::env::var("SECRET").expect("SECRET should be set"),
        kafka_stats: ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .set("delivery.timeout.ms", "5000")
            .create()
            .expect("Failed to create kafka producer")
    };
    let mut router:Router<AppState> = Router::new();
    router = route_users(router);
    router = route_posts(router);
    router = route_stats(router);
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
