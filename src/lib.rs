//! U128 helpers for Solana programs.
//!
//! [![Crates.io](https://img.shields.io/crates/v/u128)](https://crates.io/crates/u128)
//! [![License](https://img.shields.io/crates/l/u128)](https://github.com/saber-hq/u128/blob/master/LICENSE.txt)
//! [![Build Status](https://img.shields.io/github/workflow/status/saber-hq/u128/Rust/master)](https://github.com/saber-hq/u128/actions/workflows/rust.yml?query=branch%3Amaster)
//! [![Contributors](https://img.shields.io/github/contributors/saber-hq/u128)](https://github.com/saber-hq/u128/graphs/contributors)
//!
//! # Motivation
//!
//! U128 division is [very inefficient](https://github.com/solana-labs/solana/issues/19549) on
//! Solana BPF. This crate exposes a [U128] type derived using the [`uint`](https://crates.io/crates/uint) crate as a stopgap.
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::reversed_empty_ranges)]

use uint::construct_uint;
construct_uint! {
    pub struct U128(2);
}

/// Multiplies two u64's then divides by a u64.
pub fn mul_div_u64(a: u64, b: u64, divisor: u64) -> Option<u64> {
    let result = U128::from(a)
        .checked_mul(b.into())?
        .checked_div(divisor.into())?;
    if result.0[1] != 0 {
        None
    } else {
        Some(result.0[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good() {
        assert_eq!(mul_div_u64(u64::MAX, 0, 1), Some(0));
        assert_eq!(mul_div_u64(10, 10, 10), Some(10));
        assert_eq!(mul_div_u64(1_000, 3, 2), Some(1_500));
        // round down
        assert_eq!(mul_div_u64(1_000, 2, 3), Some(666));
    }

    #[test]
    fn test_div_zero() {
        assert_eq!(mul_div_u64(1, 1, 0), None);
    }

    #[test]
    fn test_overflow_u128() {
        assert_eq!(mul_div_u64(u64::MAX, u64::MAX, 0), None);
        assert_eq!(mul_div_u64(u64::MAX, u64::MAX, 1), None);
    }
}
