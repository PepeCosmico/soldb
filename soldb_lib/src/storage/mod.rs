use thiserror::Error as ThisError;

pub mod memory;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
