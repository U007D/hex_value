#![feature(const_option, const_type_name, stmt_expr_attributes)]

mod error;
mod hex_value;
mod shared_consts;

pub use error::{Error, Result};
pub use crate::hex_value::HexValue;
