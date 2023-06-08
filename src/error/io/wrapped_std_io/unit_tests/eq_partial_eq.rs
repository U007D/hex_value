use assert2::assert;
use proptest::{prop_oneof, proptest,
               strategy::{Just, Strategy}};

use super::*;

fn make_wrapped_io_err() -> impl Strategy<Value = (std::io::ErrorKind, std::io::ErrorKind)> {
    use std::io::ErrorKind::*;

    prop_oneof![Just((NotFound, PermissionDenied)),
                Just((PermissionDenied, ConnectionRefused)),
                Just((ConnectionRefused, ConnectionReset)),
                Just((ConnectionReset, ConnectionAborted)),
                Just((ConnectionAborted, NotConnected)),
                Just((NotConnected, AddrInUse)),
                Just((AddrInUse, AddrNotAvailable)),
                Just((AddrNotAvailable, BrokenPipe)),
                Just((BrokenPipe, AlreadyExists)),
                Just((AlreadyExists, WouldBlock)),
                Just((WouldBlock, InvalidInput)),
                Just((InvalidInput, InvalidData)),
                Just((InvalidData, TimedOut)),
                Just((TimedOut, WriteZero)),
                Just((WriteZero, Interrupted)),
                Just((Interrupted, Unsupported)),
                Just((Unsupported, UnexpectedEof)),
                Just((UnexpectedEof, OutOfMemory)),
                Just((OutOfMemory, Other)),
                Just((Other, NotFound)),]
}

#[test]
fn returns_expected_value() {
    // Given
    proptest!(|((error_kind_lhs, error_kind_rhs) in make_wrapped_io_err())| {
        let sut = Error(error_kind_lhs.into());
        let not_sut = Error(error_kind_rhs.into());

        /* Then */
        assert!(sut == sut);
        assert!(sut != not_sut);
    });
}
