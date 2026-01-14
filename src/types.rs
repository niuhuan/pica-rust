use thiserror::Error;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("http {status}: {message}")]
    Http { status: u16, message: String },

    #[error("deserialize error: {0}")]
    Deserialize(String),

    #[error("network error: {0}")]
    Network(String),

    #[error("invalid address: {0}")]
    InvalidAddress(String),
}
