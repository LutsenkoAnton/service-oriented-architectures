use tonic::Status;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error")]
    Database(#[from] clickhouse::error::Error),
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
            Error::Database(err) => {
                match err {
                    clickhouse::error::Error::RowNotFound => Status::not_found("Post with given id is not found"),
                    _ => Status::unknown(err.to_string())
                }
            }
            _ => Status::internal("Unknown error")
            
        }
    }
}
