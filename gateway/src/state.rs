use std::sync::{Arc, Mutex};
use crate::posts::client::PostsServerClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub grpc_client: PostsServerClient<tonic::transport::Channel>,
    pub secret: String,
}
