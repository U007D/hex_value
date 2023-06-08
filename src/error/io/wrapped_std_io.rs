#[cfg(test)]
mod unit_tests;

use std::io::Error as IoError;

use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(#[from] pub(super) IoError);

impl Clone for Error {
    fn clone(&self) -> Self { Self(IoError::new(self.0.kind(), self.0.to_string())) }
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, rhs: &Self) -> bool { self.0.kind() == rhs.0.kind() }
}
