mod args;
mod db;
mod error;
mod server;

use clap::Parser;
pub use error::{Error, Result};
use tonic::transport::Server;
use server::{posts_server::posts_server_server::PostsServerServer, MyPostsServer};
use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let addr = format!("0.0.0.0:{}", args.port);
    let server = MyPostsServer {
        pool: PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("database url env variable should be set"))
            .await?
    };
    Server::builder()
        .add_service(PostsServerServer::new(server))
        .serve(addr.parse()?)
        .await?;
    Ok(())
}
