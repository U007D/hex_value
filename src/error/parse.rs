use thiserror::Error;

use crate::shared_consts::*;

#[allow(dead_code)]
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}: '{}'", msg::ERR_ADDRESS_TOO_LARGE, 0)]
    AddressTooLarge(String),
    #[error("{}: '{}'", msg::ERR_NON_HEXADECIMAL_INPUT, 0)]
    InvalidHexadecimalInput(String),
    #[error("{}: '{}'", msg::ERR_NON_ASCII_INPUT, 0)]
    NonAsciiInput(String),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}
