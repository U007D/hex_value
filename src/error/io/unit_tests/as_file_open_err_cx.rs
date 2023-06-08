use assert2::assert;

use super::*;

#[test]
fn creates_expected_error_variant() {
    // Given
    let test_error = std::io::ErrorKind::Other;
    let expected_res = Error::FileOpenError(test_error.into());

    // When
    let res = Error::as_file_open_err_cx(test_error);

    // Then
    assert!(res == expected_res);
}
