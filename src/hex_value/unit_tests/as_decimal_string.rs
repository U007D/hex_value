use assert2::assert;

use super::*;

#[test]
fn renders_as_expected() {
    // Given
    proptest!(|(address in generate_address_type())| {
        let value = match address {
            HexValue::U16(value) => u128::from(value),
            HexValue::U32(value) => u128::from(value),
            HexValue::U64(value) => u128::from(value),
            HexValue::U128(value) => value,
        };

        let expected_res = format!("{}", value);

        /* When */
        let res = address.as_decimal_string();

        /* Then */
        assert!(res == expected_res);
    });
}
