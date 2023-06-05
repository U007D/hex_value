// This conditional compilation must be in a sub-module because code within a `cfg` block must parse
// and `impl const From...` will not parse until the `const_trait_impl` stabilizes.  Rust skips
// parsing of conditionally compiled out submodules.

pub mod msg;

pub const BITS_PER_BYTE: usize = 8;
// Define the number of hex digits in a group to be defined by a separator character.  This is used
// to format the hex string.
pub const HEX_DIGIT_GROUP_SIZE: usize = 4;
pub const HEX_DIGITS_PER_BYTE: usize = 2;
pub const HEXADECIMAL_RADIX: u32 = 16;