use std::mem::size_of;

use super::*;

#[test]
fn return_expected_values() {
    // Given
    proptest!(|(address_type in generate_address_type())| {
        #[allow(clippy::useless_conversion)]
        let (expected_address, expected_size) = match address_type {
            HexValue::U16(address) => (u128::from(address), size_of::<u16>()),
            HexValue::U32(address) => (u128::from(address), size_of::<u32>()),
            HexValue::U64(address) => (u128::from(address), size_of::<u64>()),
            HexValue::U128(address) => (u128::from(address), size_of::<u128>()),
        };

        /* When */
        let res = match address_type {
            HexValue::U16(address) => { HexValue::from(address) },
            HexValue::U32(address) => { HexValue::from(address) },
            HexValue::U64(address) => { HexValue::from(address) },
            HexValue::U128(address) => { HexValue::from(address) },
        };

        /* Then */
        assert!(res.value() == expected_address);
        assert!(res.width_bytes() == expected_size);
        assert!(res.width_bits() == expected_size.checked_mul(BITS_PER_BYTE).unwrap());
    });
}
