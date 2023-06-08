use std::io;

use assert2::assert;
use proptest::{prop_oneof, proptest,
               strategy::{Just, Strategy}};

use super::*;

fn make_wrapped_io_err() -> impl Strategy<Value = std::io::ErrorKind> {
    use std::io::ErrorKind::*;

    prop_oneof![Just(NotFound),
                Just(PermissionDenied),
                Just(ConnectionRefused),
                Just(ConnectionReset),
                Just(ConnectionAborted),
                Just(NotConnected),
                Just(AddrInUse),
                Just(AddrNotAvailable),
                Just(BrokenPipe),
                Just(AlreadyExists),
                Just(WouldBlock),
                Just(InvalidInput),
                Just(InvalidData),
                Just(TimedOut),
                Just(WriteZero),
                Just(Interrupted),
                Just(Unsupported),
                Just(UnexpectedEof),
                Just(OutOfMemory),
                Just(Other),]
}

#[test]
fn returns_expected_value() {
    // Given
    proptest!(|(error_kind in make_wrapped_io_err())| {
        let sut = Error(error_kind.into());
        let expected_res = Error(io::Error::new(error_kind, format!("{sut}")));

        /* When */
        #[allow(clippy::redundant_clone)]
        let res = sut.clone();

        /* Then */
        assert!(res == expected_res);
    });
}
