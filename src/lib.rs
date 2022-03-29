//! This crate performs fast division by a runtime constant divisor,
//! by precomputing a division factor that can be used repeatedly.
//!
//! # Example
//! ```
//! use fastdiv::FastDiv;
//!
//! let d: u32 = 3;
//! let m = d.precompute_div();
//!
//! let n1 = 4;
//! let n2 = 9;
//!
//! assert_eq!(n1 / d, n1.fast_div(m));
//! assert_eq!(n2 / d, n2.fast_div(m));
//!
//! assert_eq!(n1 % d, n1.fast_mod(m, d));
//! assert_eq!(n2 % d, n2.fast_mod(m, d));
//!
//! assert_eq!(n1 % d == 0, n1.is_multiple_of(m));
//! assert_eq!(n2 % d == 0, n2.is_multiple_of(m));
//! ```

#[inline]
const fn mul128_u32(lowbits: u64, d: u32) -> u64 {
    (lowbits as u128 * d as u128 >> 64) as u64
}
#[inline]
const fn mul128_u64(lowbits: u128, d: u64) -> u64 {
    let mut bottom_half = (lowbits & 0xFFFFFFFFFFFFFFFF) * d as u128;
    bottom_half >>= 64;
    let top_half = (lowbits >> 64) * d as u128;
    let both_halves = bottom_half + top_half;
    (both_halves >> 64) as u64
}

#[inline]
const fn compute_m_u32(d: u32) -> u64 {
    (0xFFFFFFFFFFFFFFFF / d as u64) + 1
}
#[inline]
const fn fastmod_u32(a: u32, m: u64, d: u32) -> u32 {
    let lowbits = m.wrapping_mul(a as u64);
    mul128_u32(lowbits, d) as u32
}
// for d > 1
#[inline]
const fn fastdiv_u32(a: u32, m: u64) -> u32 {
    mul128_u32(m, a) as u32
}
#[inline]
const fn is_divisible_u32(n: u32, m: u64) -> bool {
    (n as u64).wrapping_mul(m) <= m - 1
}

#[inline]
const fn compute_m_u64(d: u64) -> u128 {
    (0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF / d as u128) + 1
}
#[inline]
const fn fastmod_u64(a: u64, m: u128, d: u64) -> u64 {
    let lowbits = m.wrapping_mul(a as u128);
    mul128_u64(lowbits, d)
}
// for d > 1
#[inline]
const fn fastdiv_u64(a: u64, m: u128) -> u64 {
    mul128_u64(m, a)
}
#[inline]
const fn is_divisible_u64(n: u64, m: u128) -> bool {
    (n as u128).wrapping_mul(m) <= m - 1
}

/// Allows precomputing the division factor for fast division, modulo, and divisibility checks.
pub trait FastDiv: Copy {
    type PrecomputedDiv: Copy;
    /// Precompute the division factor from the divisor `self`.
    fn precompute_div(self) -> Self::PrecomputedDiv;
    /// Divide by the divisor, given the precomputed division factor.
    fn fast_div(self, precomputed: Self::PrecomputedDiv) -> Self;
    /// Compute the remainder of the division of `self` by the divisor, given the precomputed division factor and the divisor `d`.
    /// If the precomputed division factor does not come from the same provided divisor, the
    /// result is unspecified.
    fn fast_mod(self, precomputed: Self::PrecomputedDiv, d: Self) -> Self;
    /// Check if `self` is a multiple of the divisor, given the precomputed division factor.
    fn is_multiple_of(self, precomputed: Self::PrecomputedDiv) -> bool;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PrecomputedDivU32 {
    m: u64,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PrecomputedDivU64 {
    m: u128,
}

impl FastDiv for u32 {
    type PrecomputedDiv = PrecomputedDivU32;

    #[inline]
    fn precompute_div(self) -> Self::PrecomputedDiv {
        assert!(self > 1);
        Self::PrecomputedDiv {
            m: compute_m_u32(self),
        }
    }

    #[inline]
    fn fast_div(self, precomputed: Self::PrecomputedDiv) -> Self {
        fastdiv_u32(self, precomputed.m)
    }

    #[inline]
    fn fast_mod(self, precomputed: Self::PrecomputedDiv, d: Self) -> Self {
        fastmod_u32(self, precomputed.m, d)
    }

    #[inline]
    fn is_multiple_of(self, precomputed: Self::PrecomputedDiv) -> bool {
        is_divisible_u32(self, precomputed.m)
    }
}

impl FastDiv for u64 {
    type PrecomputedDiv = PrecomputedDivU64;

    #[inline]
    fn precompute_div(self) -> Self::PrecomputedDiv {
        assert!(self > 1);
        Self::PrecomputedDiv {
            m: compute_m_u64(self),
        }
    }

    #[inline]
    fn fast_div(self, precomputed: Self::PrecomputedDiv) -> Self {
        fastdiv_u64(self, precomputed.m)
    }

    #[inline]
    fn fast_mod(self, precomputed: Self::PrecomputedDiv, d: Self) -> Self {
        fastmod_u64(self, precomputed.m, d)
    }

    #[inline]
    fn is_multiple_of(self, precomputed: Self::PrecomputedDiv) -> bool {
        is_divisible_u64(self, precomputed.m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_u32() {
        let n: u32 = 1000;
        for j in 2..n {
            let p = j.precompute_div();
            for i in 0..n {
                assert_eq!(i.fast_mod(p, j), i % j);
                assert_eq!(i.fast_div(p), i / j);
                assert_eq!(i.is_multiple_of(p), i % j == 0);
            }
        }
    }

    #[test]
    fn div_u64() {
        let n: u64 = 1000;
        for j in 2..n {
            let p = j.precompute_div();
            for i in 0..n {
                assert_eq!(i.fast_mod(p, j), i % j);
                assert_eq!(i.fast_div(p), i / j);
                assert_eq!(i.is_multiple_of(p), i % j == 0);
            }
        }
    }
}
