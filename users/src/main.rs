mod args;
mod calls;
mod error;
mod initialize;
mod model;

use clap::Parser;
pub use error::{Error, Result};
use initialize::init_app;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    Ok(axum::serve(listener, init_app().await).await?)
}
