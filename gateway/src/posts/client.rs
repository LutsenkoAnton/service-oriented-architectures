pub use posts_server::posts_server_client::PostsServerClient;
pub use posts_server::*;

pub mod posts_server {
    tonic::include_proto!("posts");
}
