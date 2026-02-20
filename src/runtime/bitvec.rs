// bitvec.rs — width-exact bit vector library for ASL-generated Rust
//
// This file is part of the ASL-to-Rust compiler runtime.
// It is emitted alongside generated instruction files and referenced via `mod bitvec`.
//
// IMPORTANT: The generated crate root must carry:
//   #![feature(generic_const_exprs)]
//   #![allow(incomplete_features)]
// so that `slice<HI, LO>()` and `concat()` compile.

#![allow(dead_code, non_snake_case)]

/// Width-exact bit vector. Only the low `N` bits of `data` are ever set.
/// Supports up to 128 bits (backed by u128).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BitVec<const N: usize> {
    data: u128,
}

// ── Internal helper ──────────────────────────────────────────────────────────

/// Bit mask for `n` bits: `(1 << n) - 1`.
/// Handles n == 0 (→ 0) and n >= 128 (→ u128::MAX).
const fn mask(n: usize) -> u128 {
    if n == 0 {
        0
    } else if n >= 128 {
        u128::MAX
    } else {
        (1u128 << n) - 1
    }
}

// ── Constructors ─────────────────────────────────────────────────────────────

impl<const N: usize> BitVec<N> {
    /// All-zeros bit vector.
    pub fn zero() -> Self {
        BitVec { data: 0 }
    }

    /// Construct from a `u64` value, masking to `N` bits.
    pub fn from_u64(v: u64) -> Self {
        BitVec { data: (v as u128) & mask(N) }
    }

    /// Construct from a `u128` value, masking to `N` bits.
    pub fn from_u128(v: u128) -> Self {
        BitVec { data: v & mask(N) }
    }

    /// Construct from a signed `i128` value, masking to `N` bits.
    /// The two's-complement bit pattern is preserved.
    pub fn from_i128(v: i128) -> Self {
        BitVec { data: (v as u128) & mask(N) }
    }
}

// ── Conversions ───────────────────────────────────────────────────────────────

impl<const N: usize> BitVec<N> {
    /// Return all `N` bits as a `u128` (zero-extended).
    pub fn to_u128(&self) -> u128 {
        self.data
    }

    /// Return the low 64 bits as a `u64`.
    /// Panics if `N > 64` to prevent silent truncation.
    pub fn to_u64(&self) -> u64 {
        assert!(N <= 64, "BitVec<{}> is wider than u64; use to_u128()", N);
        self.data as u64
    }

    /// Sign-extend to `i128`.
    /// Bit `N-1` is the sign bit.
    pub fn to_i128_signed(&self) -> i128 {
        if N == 0 {
            return 0;
        }
        let sign_bit = (self.data >> (N - 1)) & 1;
        if sign_bit == 0 {
            self.data as i128
        } else {
            // Fill upper bits with 1s
            let extended = self.data | !mask(N);
            extended as i128
        }
    }
}

// ── Bit read / write ──────────────────────────────────────────────────────────

impl<const N: usize> BitVec<N> {
    /// Read a single bit at position `i` (0 = LSB).
    /// Panics if `i >= N`.
    pub fn bit(&self, i: usize) -> bool {
        assert!(i < N, "bit index {} out of range for BitVec<{}>", i, N);
        (self.data >> i) & 1 == 1
    }

    /// Write a single bit at position `i`.
    /// Panics if `i >= N`.
    pub fn set_bit(&mut self, i: usize, val: bool) {
        assert!(i < N, "bit index {} out of range for BitVec<{}>", i, N);
        if val {
            self.data |= 1u128 << i;
        } else {
            self.data &= !(1u128 << i);
        }
    }
}

// ── Bit-slice read (runtime hi/lo) ────────────────────────────────────────────

impl<const N: usize> BitVec<N> {
    /// Extract bits `[hi:lo]` (inclusive) as a `u128`.
    /// Corresponds to ASL `x<hi:lo>` where `hi` and `lo` are runtime values.
    ///
    /// Returns the value of the runtime version of `slice<HI, LO>()`, but
    /// without a statically-known width. Use `slice()` when HI/LO are const.
    pub fn slice_rt(&self, hi: usize, lo: usize) -> u128 {
        assert!(hi < N, "hi={} out of range for BitVec<{}>", hi, N);
        assert!(lo <= hi, "lo={} > hi={}", lo, hi);
        let width = hi - lo + 1;
        (self.data >> lo) & mask(width)
    }

    /// Extract a single bit as a `u128` 0 or 1.
    /// Convenience wrapper for `bit(i) as u128`.
    pub fn bit_u128(&self, i: usize) -> u128 {
        self.bit(i) as u128
    }
}

// ── Bit-slice write (runtime hi/lo) ───────────────────────────────────────────

impl<const N: usize> BitVec<N> {
    /// Write bits `[hi:lo]` (inclusive) from the low bits of `val`.
    /// Corresponds to ASL `x<hi:lo> = val`.
    pub fn set_slice(&mut self, lo: usize, hi: usize, val: u128) {
        assert!(hi < N, "hi={} out of range for BitVec<{}>", hi, N);
        assert!(lo <= hi, "lo={} > hi={}", lo, hi);
        let width = hi - lo + 1;
        let m = mask(width);
        self.data &= !(m << lo);          // clear the target bits
        self.data |= (val & m) << lo;     // insert the new value
    }
}

// ── Const-generic slice and concat (needs nightly generic_const_exprs) ────────

impl<const N: usize> BitVec<N> {
    /// Extract bits `[HI:LO]` as a `BitVec<{HI-LO+1}>`.
    /// Corresponds to ASL `x<HI:LO>` where HI and LO are compile-time constants.
    ///
    /// Requires `#![feature(generic_const_exprs)]` at the crate root.
    pub fn slice<const HI: usize, const LO: usize>(&self) -> BitVec<{ HI - LO + 1 }>
    where
        [(); HI - LO + 1]: Sized,
    {
        assert!(HI < N, "HI={} out of range for BitVec<{}>", HI, N);
        assert!(LO <= HI, "LO={} > HI={}", LO, HI);
        let width = HI - LO + 1;
        BitVec { data: (self.data >> LO) & mask(width) }
    }

    /// Concatenate: `self` provides the high bits, `other` provides the low bits.
    /// Corresponds to ASL `self:other` (or `self ++ other`).
    ///
    /// Result width is `N + M`.
    /// Requires `#![feature(generic_const_exprs)]` at the crate root.
    pub fn concat<const M: usize>(self, other: BitVec<M>) -> BitVec<{ N + M }>
    where
        [(); N + M]: Sized,
    {
        BitVec { data: (self.data << M) | other.data }
    }
}

// ── Debug / Display ───────────────────────────────────────────────────────────

impl<const N: usize> core::fmt::Debug for BitVec<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BitVec<{}>({:#034x})", N, self.data)
    }
}

impl<const N: usize> core::fmt::Binary for BitVec<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:0>width$b}", self.data, width = N)
    }
}

// ── Default ───────────────────────────────────────────────────────────────────

impl<const N: usize> Default for BitVec<N> {
    fn default() -> Self {
        BitVec::zero()
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let v: BitVec<8> = BitVec::zero();
        assert_eq!(v.to_u128(), 0);
    }

    #[test]
    fn test_from_u64_masks() {
        // from_u64 should mask to N bits
        let v: BitVec<4> = BitVec::from_u64(0xFF);
        assert_eq!(v.to_u128(), 0xF);
    }

    #[test]
    fn test_bit_read_write() {
        let mut v: BitVec<8> = BitVec::zero();
        v.set_bit(3, true);
        assert!(v.bit(3));
        assert!(!v.bit(2));
        assert_eq!(v.to_u128(), 0b0000_1000);
    }

    #[test]
    fn test_slice_rt() {
        let v: BitVec<8> = BitVec::from_u64(0b1010_1010);
        assert_eq!(v.slice_rt(7, 4), 0b1010);
        assert_eq!(v.slice_rt(3, 0), 0b1010);
        assert_eq!(v.slice_rt(1, 1), 1);
    }

    #[test]
    fn test_set_slice() {
        let mut v: BitVec<8> = BitVec::from_u64(0b1111_1111);
        v.set_slice(4, 7, 0b0000);
        assert_eq!(v.to_u128(), 0b0000_1111);
    }

    #[test]
    fn test_slice_const() {
        let v: BitVec<8> = BitVec::from_u64(0b1010_1010);
        let hi: BitVec<4> = v.slice::<7, 4>();
        let lo: BitVec<4> = v.slice::<3, 0>();
        assert_eq!(hi.to_u128(), 0b1010);
        assert_eq!(lo.to_u128(), 0b1010);
    }

    #[test]
    fn test_concat() {
        let hi: BitVec<4> = BitVec::from_u64(0b1100);
        let lo: BitVec<4> = BitVec::from_u64(0b0011);
        let combined: BitVec<8> = hi.concat(lo);
        assert_eq!(combined.to_u128(), 0b1100_0011);
    }

    #[test]
    fn test_signed_extension() {
        // 0b1000 in 4 bits = -8 in two's complement
        let v: BitVec<4> = BitVec::from_u64(0b1000);
        assert_eq!(v.to_i128_signed(), -8);

        // 0b0111 in 4 bits = 7
        let v: BitVec<4> = BitVec::from_u64(0b0111);
        assert_eq!(v.to_i128_signed(), 7);
    }

    #[test]
    fn test_128_bit() {
        let v: BitVec<128> = BitVec::from_u128(u128::MAX);
        assert_eq!(v.to_u128(), u128::MAX);
    }
}
