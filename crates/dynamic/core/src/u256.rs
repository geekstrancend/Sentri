//! Minimal big-endian 256-bit unsigned integer helpers.
//!
//! Invariant oracles need to compare and sum values decoded from EVM-style
//! 32-byte return data without pulling in a full bignum dependency. These
//! operate directly on `[u8; 32]` in big-endian byte order (the ABI encoding
//! used for `uint256`), which keeps this crate chain-agnostic and dependency-free.

/// Byte-for-byte lexicographic comparison is equivalent to numeric comparison
/// for big-endian unsigned integers of equal width.
pub fn u256_lt(a: &[u8; 32], b: &[u8; 32]) -> bool {
    a < b
}

/// Wrapping addition of two big-endian `uint256` values.
pub fn u256_add(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut carry: u16 = 0;
    for i in (0..32).rev() {
        let sum = a[i] as u16 + b[i] as u16 + carry;
        out[i] = (sum & 0xFF) as u8;
        carry = sum >> 8;
    }
    out
}

/// Renders a `uint256` byte array as a decimal string, for human-readable
/// violation messages and PoC output. Implemented via repeated divide-by-10
/// on the byte array since we deliberately avoid a bignum dependency.
pub fn u256_to_decimal(value: &[u8; 32]) -> String {
    let mut digits: Vec<u8> = Vec::new();
    let mut work = *value;
    loop {
        let mut remainder: u32 = 0;
        let mut any_nonzero = false;
        for byte in work.iter_mut() {
            let acc = (remainder << 8) | (*byte as u32);
            *byte = (acc / 10) as u8;
            remainder = acc % 10;
            if *byte != 0 {
                any_nonzero = true;
            }
        }
        digits.push(remainder as u8);
        if !any_nonzero {
            break;
        }
    }
    digits
        .iter()
        .rev()
        .map(|d| (b'0' + d) as char)
        .collect::<String>()
}

pub fn u256_from_u128(value: u128) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[16..32].copy_from_slice(&value.to_be_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lt_is_numeric_not_just_byte_order_for_equal_width() {
        let small = u256_from_u128(5);
        let large = u256_from_u128(300);
        assert!(u256_lt(&small, &large));
        assert!(!u256_lt(&large, &small));
    }

    #[test]
    fn add_carries_across_byte_boundary() {
        let a = u256_from_u128(255);
        let b = u256_from_u128(1);
        let sum = u256_add(&a, &b);
        assert_eq!(u256_to_decimal(&sum), "256");
    }

    #[test]
    fn add_wraps_on_overflow_rather_than_panicking() {
        let max = [0xFFu8; 32];
        let one = u256_from_u128(1);
        let wrapped = u256_add(&max, &one);
        assert_eq!(wrapped, [0u8; 32]);
    }

    #[test]
    fn decimal_roundtrip_for_zero_and_large_values() {
        assert_eq!(u256_to_decimal(&[0u8; 32]), "0");
        assert_eq!(u256_to_decimal(&u256_from_u128(123_456_789)), "123456789");
    }
}
