extern crate ureq;
extern crate serde_json;

use std::fmt;

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
        Self {
            message: str,
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        match err {
            ureq::Error::Status(state, resp) => {
                Self {
                    message: [
                        "HTTP CODE NOT 200".to_string(),
                        format!("{}", state),
                        resp.into_string().unwrap(),
                    ].join(" : ")
                }
            }
            ureq::Error::Transport(t) => {
                Self {
                    message: t.to_string()
                }
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            message: err.to_string()
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self {
            message: err.to_string()
        }
    }
}

