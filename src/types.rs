use std::fmt;

pub type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;

pub struct Error {
    message: String,
}

impl Error {
    pub fn with_string(string: String) -> Self {
        Self { message: string }
    }
    pub fn with_str(str: &str) -> Self {
        Self::with_string(str.to_string())
    }
}

impl std::error::Error for Error {}

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
        Self::with_str(str)
    }
}

impl From<String> for Error {
    fn from(str: String) -> Self {
        Self::with_string(str)
    }
}
