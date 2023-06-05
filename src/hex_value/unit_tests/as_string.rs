use assert2::assert;

use super::*;

#[test]
fn renders_as_expected() {
    // Given
    proptest!(|(hex_value in generate_address_type())| {
        let value = match hex_value {
            HexValue::U16(value) => u128::from(value),
            HexValue::U32(value) => u128::from(value),
            HexValue::U64(value) => u128::from(value),
            HexValue::U128(value) => value,
        };

        let expected_res = format!("{value:x}");

        /* When */
        let res = hex_value.as_string();

        /* Then */
        assert!(res == expected_res);
    });
}
