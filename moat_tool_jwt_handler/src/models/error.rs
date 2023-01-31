use core::fmt;
use thiserror::Error;


#[derive(Error)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
    #[error("key not found")]
    KeyNotFoundError
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongCredentialsError => write!(f, "WrongCredentialsError"),
            Self::JWTTokenError => write!(f, "JWTTokenError"),
            Self::JWTTokenCreationError => write!(f, "JWTTokenCreationError"),
            Self::NoAuthHeaderError => write!(f, "NoAuthHeaderError"),
            Self::InvalidAuthHeaderError => write!(f, "InvalidAuthHeaderError"),
            Self::NoPermissionError => write!(f, "NoPermissionError"),
            Self::KeyNotFoundError => write!(f, "KeyNotFoundError"),
        }
    }
}