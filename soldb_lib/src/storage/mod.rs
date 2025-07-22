use thiserror::Error as ThisError;

pub mod memory;
pub mod persistence;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("Encode Error")]
    EncodeError(#[from] bincode::error::EncodeError),
    #[error("Decode Error")]
    DecodeError(#[from] bincode::error::DecodeError),
}

pub type Result<T> = std::result::Result<T, Error>;
