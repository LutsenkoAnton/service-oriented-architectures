use tonic::Status;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound(String),
    #[error("Forbidden")]
    Forbidden(String),
    // #[error("API error")]
    // API(#[from] axum::Error),
    #[error("IO problem, most likely with network")]
    IO(#[from] std::io::Error),
    #[error("Error while parsing socket")]
    Socket(#[from] std::net::AddrParseError),
    #[error("Something wrong with building RPC")]
    RPC(#[from] tonic::transport::Error),
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound(err) => Status::not_found(err),
            Error::Forbidden(err) => Status::permission_denied(err),
            Error::Database(err) => {
                match err {
                    sqlx::Error::RowNotFound => Status::not_found("Post with given id is not found"),
                    _ => Status::unknown("Unknown error")
                }
            }
            _ => Status::internal("Unknown error")
            
        }
    }
}
