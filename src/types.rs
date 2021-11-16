use std::fmt;
use std::net::AddrParseError;

pub type Result<A> = std::result::Result<A, Error>;

pub struct Error {
    message: String,
}

impl Error {
    pub fn to_string(&self) -> String {
        self.message.clone()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.message, f)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.message, f)
    }
}

impl From<&str> for Error {
    fn from(str: &str) -> Self {
        Self {
            message: str.to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(str: String) -> Self {
        Self { message: str }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<AddrParseError> for Error {
    fn from(error: AddrParseError) -> Self {
        Self { message: error.to_string() }
    }
}
