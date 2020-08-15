//! Miscellaneous bit operations for any [`Integer`].
//!
//! See the [`BitOps`] trait for examples.
//!
//! [`Integer`]: ../num_integer/trait.Integer.html
//! [`BitOps`]: ./trait.BitOps.html

use std::ops::{BitAnd, Shl, Shr};

use num_integer::Integer;

/// Miscellaneous bit operations for any [`Integer`].
///
/// # Examples
///
/// ```
/// use bitops::BitOps;
///
/// let x = 0b1010_1011_0000_1100; // 0xab0c
/// let flag = 0b1000;
///
/// assert!(flag.is_flag());
/// assert!(flag.is_bit_set(3));
///
/// assert!(x.is_flag_set(flag));
/// assert_eq!(x.bits_as_int(8, 4), 0xb);
/// ```
///
/// [`Integer`]: ../num_integer/trait.Integer.html
pub trait BitOps:
    Copy + Integer + BitAnd<Output = Self> + Shl<Output = Self> + Shr<Output = Self> + From<u8>
{
    /// Returns whether this number only has one bit set.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitops::BitOps;
    ///
    /// assert!(0b1000.is_flag());
    /// assert!(!0b1001.is_flag());
    /// ```
    #[inline]
    fn is_flag(&self) -> bool {
        *self > Self::zero() && (*self & (*self - Self::one())) == Self::zero()
    }

    /// Returns whether the given bit number is set.
    ///
    /// # Panics
    ///
    /// Panics if `bit` is greater than the number of bits in this Integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitops::BitOps;
    ///
    /// assert!(0b1000.is_bit_set(3));
    /// ```
    #[inline]
    fn is_bit_set(&self, bit: u8) -> bool {
        self.is_flag_set(Self::one() << Self::from(bit))
    }

    /// Returns whether the given flag is set.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitops::BitOps;
    ///
    /// assert!(0b11010.is_flag_set(0b11000));
    /// ```
    #[inline]
    fn is_flag_set(&self, flag: Self) -> bool {
        *self & flag > Self::zero()
    }

    /// Returns a number with bit 0 starting at the given bit and up to `count` left most bits.
    /// This is basically a right shift by `bit` and masked with `(1 << count) - 1`.
    ///
    /// # Panics
    ///
    /// Panics if `bit` is greater than the number of bits in this Integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitops::BitOps;
    ///
    /// assert_eq!(0xab000.bits_as_int(12, 8), 0xab);
    /// ```
    #[inline]
    fn bits_as_int(&self, bit: u8, count: u8) -> Self {
        (*self >> Self::from(bit)) & ((Self::one() << Self::from(count)) - Self::one())
    }
}

/// Implements the [`BitOps`] trait for all types that meet the requirements.
impl<N> BitOps for N where
    N: Copy + Integer + BitAnd<Output = Self> + Shl<Output = Self> + Shr<Output = Self> + From<u8>
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_zero() {
        assert!(!0.is_flag());
    }

    #[test]
    fn flag_set_blank() {
        assert!(!0x0000.is_flag_set(0));
    }

    #[test]
    fn bits_at_zero() {
        assert_eq!(0xabcd.bits_as_int(0, 8), 0xcd);
    }

    #[test]
    #[should_panic]
    fn bits_overflow() {
        0u16.bits_as_int(16, 0);
    }
}
