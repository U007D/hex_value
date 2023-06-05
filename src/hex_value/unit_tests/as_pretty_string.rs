use std::mem::size_of_val;

use assert2::assert;

use super::*;

#[test]
fn renders_as_expected() {
    // Given
    proptest!(|(hex_value in generate_address_type())| {
        let (value, width_bytes) = match hex_value {
            HexValue::U16(value) => (u128::from(value), size_of_val(&value)),
            HexValue::U32(value) => (u128::from(value), size_of_val(&value)),
            HexValue::U64(value) => (u128::from(value), size_of_val(&value)),
            HexValue::U128(value) => (value, size_of_val(&value)),
        };

        // Generate hex string of correct width padded with leading zeros
        let utf8_byte_vec = format!(
            "{:0width$x}",
            value,
            width = width_bytes.checked_mul(HEX_DIGITS_PER_BYTE).unwrap()
        ).as_bytes()

        // Group the string into `HEX_GROUP_SIZE`-character groups
        .rchunks(HEX_DIGIT_GROUP_SIZE).enumerate().rev().fold(Vec::new(), |mut acc, (i, group)| {
            acc.extend(group);
            (i > 0).then(|| acc.push(u8::try_from('_').unwrap()));
            acc
        });

        // Add the "0x" prefix and width suffix
        #[allow(clippy::arithmetic_side_effects)]
        let expected_res =
            String::from("0x")
            + std::str::from_utf8(&utf8_byte_vec).unwrap()
            + ":"
            + &width_bytes.checked_mul(BITS_PER_BYTE).unwrap().to_string();

        /* When */
        let res = hex_value.as_pretty_string();

        /* Then */
        assert!(res == expected_res);
    });
}
