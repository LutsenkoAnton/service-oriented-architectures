use crate::posts::client::PostsServerClient;
use rdkafka::producer::FutureProducer;

#[derive(Clone)]
pub struct AppState {
    pub grpc_client_posts: PostsServerClient<tonic::transport::Channel>,
    pub secret: String,
    pub kafka_stats: FutureProducer,
}
