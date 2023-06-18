use std::num::ParseIntError;
use thiserror::Error;

/// Result
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Error
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    #[error(transparent)]
    ParseAtom(#[from] atom::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}
