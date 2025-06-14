mod args;
mod db;
mod error;
mod server;

use clap::Parser;
pub use error::{Error, Result};
use tonic::transport::Server;
use server::{stats_server::stats_server::StatsServer, MyStatsServer};
use clickhouse::Client;


#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let addr = format!("0.0.0.0:{}", args.port);
    let database_url = std::env::var("DATABASE_URL").expect("Clickhouse url should be set");
    let user = std::env::var("CLICKHOUSE_USER").expect("Clickhouse username should be set");
    let pass = std::env::var("CLICKHOUSE_PASSWORD").expect("Clickhouse password should be set");
    let server = MyStatsServer {
        client: Client::default()
            .with_url(database_url)
            .with_user(user)
            .with_password(pass)
    };
    Server::builder()
        .add_service(StatsServer::new(server))
        .serve(addr.parse()?)
        .await?;
    Ok(())
}
