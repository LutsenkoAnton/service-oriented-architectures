pub use stats::stats_client::StatsClient;
pub use stats::*;

pub mod stats {
    tonic::include_proto!("stats");
}
