#[cfg(test)]
mod unit_tests;

use std::{fmt::{Display, Formatter, Result as FmtResult},
          mem::size_of,
          ops::Not};

use bool_ext::BoolExt;

use crate::{error::parse::Error, shared_consts::*};

/// Enum representing a hexadecimal value, preserving the input value's width.
/// `HexValue` may be used wherever hexadecimal representation is required.
/// One use is as a host machine's typed representation of a memory address on a remote machine,
/// such as a flash memory base address on a connected embedded device.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HexValue {
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl HexValue {
    /// Returns the address value.
    #[must_use]
    pub fn value(&self) -> u128 {
        match self {
            Self::U16(value) => u128::from(*value),
            Self::U32(value) => u128::from(*value),
            Self::U64(value) => u128::from(*value),
            Self::U128(value) => *value,
        }
    }

    /// Print the address as an unformatted hexadecimal string.
    #[must_use]
    pub fn as_string(&self) -> String { format!("{:x}", self.value()) }

    /// Pretty print the address as a hexadecimal string with underscores and leading zeros followed
    /// by its width in bits.
    #[allow(clippy::assertions_on_constants)]
    #[must_use]
    pub fn as_pretty_string(&self) -> String {
        let padded_address = self.as_padded_string();

        // Invariants:
        debug_assert!(HEX_DIGIT_GROUP_SIZE > 0);
        debug_assert!(padded_address.is_ascii());
        debug_assert!(padded_address.len() % HEX_DIGIT_GROUP_SIZE == 0);

        #[rustfmt::skip]
        padded_address.as_bytes()
            .chunks_exact(HEX_DIGIT_GROUP_SIZE)
            .enumerate()
            .fold(String::from("0x"), |mut acc, (i, chunk)| {
                (i > 0).then(|| acc.push('_'));
                acc.push_str(std::str::from_utf8(chunk).unwrap_or_else(|err| {
                    unreachable!("Internal error: Invalid UTF-8 encountered in ASCII data \
                                         sourced from UTF-8 string.  Details: {:?}", err)
                }));
                acc
            })
            + ":"
            + &self.width_bits().to_string()
    }

    /// Print the address as an unformatted string of decimal digits.
    #[must_use]
    pub fn as_decimal_string(&self) -> String { self.value().to_string() }

    /// Print the address as a formatted string of hex digits with leading zeros padding to the full
    /// address width.
    #[must_use]
    pub fn as_padded_string(&self) -> String {
        format!("{:0width$x}",
                self.value(),
                width = self.width_bytes()
                            .checked_mul(HEX_DIGITS_PER_BYTE)
                            .unwrap_or_else(|| unreachable!("Internal Error: Number of hexadecimal \
                                                         digits in `Address` overflowed `usize`.")))
    }

    /// Returns the width of the address value in bytes.
    #[must_use]
    pub const fn width_bytes(&self) -> usize {
        match self {
            Self::U16(_) => size_of::<u16>(),
            Self::U32(_) => size_of::<u32>(),
            Self::U64(_) => size_of::<u64>(),
            Self::U128(_) => size_of::<u128>(),
        }
    }

    /// Returns the width of the address value in bits.
    #[must_use]
    pub const fn width_bits(&self) -> usize {
        self.width_bytes()
            .checked_mul(BITS_PER_BYTE)
            // TODO: Switch to `.unwrap_or_else(|| unreachable!())` when `.unwrap_or_else()` is `const`
            .expect("Internal Error: Address width overflowed `usize`")
    }
}

impl From<u16> for HexValue {
    fn from(value: u16) -> Self { Self::U16(value) }
}

impl From<u32> for HexValue {
    fn from(value: u32) -> Self { Self::U32(value) }
}

impl From<u64> for HexValue {
    fn from(value: u64) -> Self { Self::U64(value) }
}

impl From<u128> for HexValue {
    fn from(value: u128) -> Self { Self::U128(value) }
}

impl Display for HexValue {
    #[allow(clippy::assertions_on_constants)]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match f.alternate() {
            false => write!(f, "0x{}", self.as_string()),
            true => write!(f, "{}", self.as_pretty_string()),
        }
    }
}

impl TryFrom<&str> for HexValue {
    type Error = Error;

    /// Attempts to parse a string containing hexadecimal characters into a `Value`.  The input may
    /// optionally be prefixed with `0x` and may contain `_` separators.
    fn try_from(raw_input: &str) -> Result<Self, Self::Error> {
        // Determine if input is pure ASCII (all hexadecimal digits are ASCII)
        raw_input.is_ascii().or_try_do(|| Err(Error::NonAsciiInput(raw_input.to_string())))?;

        // Strip optional leading `0x`, underscore separators and leading zeros
        let input = {
            let tmp =
                raw_input.trim().to_ascii_lowercase().trim_start_matches("0x").replace('_', "");
            // Remove all leading zeros except for one
            let tmp = tmp.trim_start_matches('0');
            // If `raw_input` contained at least one zero but the sanitized input is empty,
            // `trim_start_matches('0')` must have removed all zeros but we want to restore a zero.
            (tmp.is_empty() && raw_input.contains('0')).then_some("0").unwrap_or(tmp).to_string()
        };

        // Reject empty strings or strings containing arithmetic operators (e.g. `+`, `-` which may
        // be accepted by `from_str_radix`, but should not be permitted for hex addresses).
        (input.is_empty() || input.contains(|c: char| c.is_ascii_hexdigit().not()))
             .and_try_do(|| Err(Error::InvalidHexadecimalInput(raw_input.to_string())))?;

        // Set the width based on the length of the input
        let value = match input.len() {
            1..=4 => Self::U16(u16::from_str_radix(&input, HEXADECIMAL_RADIX)?),
            5..=8 => Self::U32(u32::from_str_radix(&input, HEXADECIMAL_RADIX)?),
            9..=16 => Self::U64(u64::from_str_radix(&input, HEXADECIMAL_RADIX)?),
            17..=32 => Self::U128(u128::from_str_radix(&input, HEXADECIMAL_RADIX)?),
            _ => Err(Error::AddressTooLarge(raw_input.to_string()))?,
        };
        Ok(value)
    }
}
