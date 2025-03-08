pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("API error")]
    API(#[from] axum::Error),
    #[error("IO problem, most likely with network")]
    IO(#[from] std::io::Error),
    #[error("Password hashing failed")]
    Argon2(argon2::password_hash::Error),
}
