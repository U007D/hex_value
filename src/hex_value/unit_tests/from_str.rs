use std::ops::Not;

use assert2::let_assert;
use regex::Regex;

#[allow(unused_imports)]
use super::*;

#[test]
fn rejects_empty_string() {
    // Given
    let empty_string = "";

    // When
    let res = HexValue::from_str(empty_string);

    // Then
    let_assert!(Err(Error::InvalidHexadecimalInput(_address)) = res);
}

#[test]
fn rejects_non_ascii_string() {
    // Given
    fn non_ascii_string() -> impl Strategy<Value = String> {
        any::<String>().prop_filter("reject pure ascii strings", move |s| s.is_ascii().not())
    }

    proptest!(|(s in non_ascii_string())| {
        // When
        let res = HexValue::from_str(s.as_ref());

        // Then
        let_assert!(Err(Error::NonAsciiInput(_non_ascii_string)) = res);
    });
}

#[test]
fn rejects_ascii_string_containing_invalid_hex() {
    // Given
    const HEX_STRING: &str = "^\\s*(?:0[xX])?[_]*(?:[0-9a-fA-F][_]*){1,}\\s*$";

    fn invalid_hex_string(re: Regex) -> impl Strategy<Value = String> {
        any::<String>().prop_filter("reject valid hex strings", move |s| {
                           s.is_ascii()
                           && s.trim()
                               .to_ascii_lowercase()
                               .trim_start_matches("0x")
                               .replace('_', "")
                               .is_empty()
                               .not()
                           && re.is_match(s).not()
                       })
    }
    let re = Regex::new(HEX_STRING).unwrap();

    proptest!(|(s in invalid_hex_string(re))| {
        // When
        let res = HexValue::from_str(s.as_ref());

        // Then
        let_assert!(Err(Error::InvalidHexadecimalInput(_address)) = res);
    });
}

#[test]
fn rejects_valid_hex_string_greater_than_u128_max() {
    // Given
    const BIG_HEX_REGEX: &str = "[1-9a-fA-F][0-9a-fA-F]{32,}";

    fn hex_string() -> impl Strategy<Value = String> { string_regex(BIG_HEX_REGEX).unwrap() }

    proptest!(|(s in hex_string())| {
        // When
        let res = HexValue::from_str(s.as_ref());

        // Then
        let_assert!(Err(Error::AddressTooLarge(_input)) = res);
    });
}

#[test]
fn accepts_valid_hex_string() {
    // Given
    const HEX_REGEX: &str = "[0-9a-fA-F]{1, 32}";

    fn hex_string() -> impl Strategy<Value = String> { string_regex(HEX_REGEX).unwrap() }

    proptest!(|(s in hex_string())| {
        // When
        let res = HexValue::from_str(s.as_ref());

        // Then
        let_assert!(Ok(_address) = res);
    });
}

#[test]
fn accepts_valid_hex_string_containing_optional_prefix_and_separators() {
    // Given
    const HEX_REGEX: &str = "(?:0[xX])?[_]*(?:[0-9a-fA-F][_]*){1, 32}";

    fn hex_string() -> impl Strategy<Value = String> { string_regex(HEX_REGEX).unwrap() }

    proptest!(|(s in hex_string())| {
        // When
        let res = HexValue::from_str(s.as_ref());

        // Then
        let_assert!(Ok(_address) = res);
    });
}
