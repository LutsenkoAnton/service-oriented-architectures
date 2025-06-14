use crate::posts::client::PostsServerClient;
use crate::stats::client::StatsClient;
use rdkafka::producer::FutureProducer;

#[derive(Clone)]
pub struct AppState {
    pub grpc_client_posts: PostsServerClient<tonic::transport::Channel>,
    pub grpc_client_stats: StatsClient<tonic::transport::Channel>,
    pub users_url: String,
    pub secret: String,
    pub kafka_stats: FutureProducer,
}
