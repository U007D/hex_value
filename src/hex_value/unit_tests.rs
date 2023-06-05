#![allow(clippy::unwrap_used)]

pub use proptest::{prelude::*, string::string_regex};

#[allow(unused_imports)]
use super::*;

mod value_and_width_bits_and_width_bytes;
mod as_string;
mod as_pretty_string;
mod as_decimal_string;
mod display;
mod try_from;

fn generate_address_type() -> impl Strategy<Value =HexValue> {
    prop_oneof![any::<u16>().prop_map(HexValue::U16),
                any::<u32>().prop_map(HexValue::U32),
                any::<u64>().prop_map(HexValue::U64),
                any::<u128>().prop_map(HexValue::U128),]
}
