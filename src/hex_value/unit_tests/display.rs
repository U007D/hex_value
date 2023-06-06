use assert2::assert;

use super::*;

#[test]
fn standard_format_displays_as_expected() {
    // Given
    let expected_res = "0xd020";
    let address = HexValue::from(53280_u32);

    // When
    let res = format!("{address}");

    // Then
    assert!(res == expected_res);
}

#[test]
fn alternate_format_displays_as_expected() {
    // Given
    let expected_res = "0x8000_0000:32";
    let address = HexValue::from(2_147_483_648_u32);

    // When
    let res = format!("{address:#}");

    // Then
    assert!(res == expected_res);
}
