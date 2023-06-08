#![allow(clippy::unwrap_used)]

use super::*;

impl From<std::io::ErrorKind> for wrapped_std_io::Error {
    fn from(error_kind: std::io::ErrorKind) -> Self { Self(std::io::Error::from(error_kind)) }
}

mod as_file_create_err_cx;
mod as_file_open_err_cx;
mod as_file_read_err_cx;
mod as_file_write_err_cx;
