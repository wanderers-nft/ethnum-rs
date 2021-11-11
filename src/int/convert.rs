//! Module contains conversions for [`I256`] to and from primimitive types.

use crate::U256;

use super::I256;
use core::{convert::TryFrom, mem, num::TryFromIntError};

macro_rules! impl_from {
    ($($t:ty),* $(,)?) => {$(
        impl From<$t> for I256 {
            #[inline]
            fn from(value: $t) -> Self {
                value.as_i256()
            }
        }
    )*};
}

impl_from! {
    bool,
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
}

/// This trait defines `as` conversions (casting) from primitive types to
/// [`I256`].
///
/// [`I256`]: struct.I256.html
///
/// # Examples
///
/// Note that in Rust casting from a negative signed integer sign to a larger
/// unsigned interger sign extends. Additionally casting a floating point value
/// to an integer is a saturating operation, with `NaN` converting to `0`. So:
///
/// ```
/// # use ethnum::{I256, AsI256};
/// assert_eq!((-1i32).as_i256(), I256::MAX);
/// assert_eq!(u32::MAX.as_i256(), 0xffffffff);
///
/// assert_eq!(f64::NEG_INFINITY.as_i256(), 0);
/// assert_eq!((-1.0f64).as_i256(), 0);
/// assert_eq!(f64::INFINITY.as_i256(), I256::MAX);
/// assert_eq!(2.0f64.powi(257).as_i256(), I256::MAX);
/// assert_eq!(f64::NAN.as_i256(), 0);
/// ```
pub trait AsI256 {
    /// Perform an `as` conversion to a [`I256`].
    ///
    /// [`I256`]: struct.I256.html
    #[allow(clippy::wrong_self_convention)]
    fn as_i256(self) -> I256;
}

impl AsI256 for I256 {
    #[inline]
    fn as_i256(self) -> I256 {
        self
    }
}

impl AsI256 for U256 {
    #[inline]
    fn as_i256(self) -> I256 {
        let (hi, lo) = self.into_words();
        I256::from_words(hi as _, lo as _)
    }
}

macro_rules! impl_as_i256 {
    ($($t:ty),* $(,)?) => {$(
        impl AsI256 for $t {
            #[inline]
            fn as_i256(self) -> I256 {
                #[allow(unused_comparisons)]
                let hi = if self >= 0 { 0 } else { !0 };
                I256::from_words(hi, self as _)
            }
        }
    )*};
}

impl_as_i256! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
    isize, usize,
}

impl AsI256 for bool {
    #[inline]
    fn as_i256(self) -> I256 {
        I256::new(self as _)
    }
}

macro_rules! impl_as_i256_float {
    ($($t:ty [$b:ty]),* $(,)?) => {$(
        impl AsI256 for $t {
            #[inline]
            fn as_i256(self) -> I256 {
                // The conversion follows roughly the same rules as converting
                // `f64` to other primitive integer types:
                // - `NaN` => `0`
                // - `(-∞, I256::MIN]` => `I256::MIN`
                // - `(I256::MIN, I256::MAX]` => `value as I256`
                // - `(I256::MAX, +∞)` => `I256::MAX`

                const M: u32 = <$t>::MANTISSA_DIGITS - 1;
                const MAN_MASK: $b = !(!0 << M);
                const MAN_ONE: $b = 1 << M;
                const EXP_MASK: $b = !0 >> <$t>::MANTISSA_DIGITS;
                const EXP_OFFSET: $b = EXP_MASK / 2;
                const ABS_MASK: $b = !0 >> 1;
                const SIG_MASK: $b = !ABS_MASK;

                let abs = <$t>::from_bits(self.to_bits() & ABS_MASK);
                if abs >= 1.0 {
                    let bits = abs.to_bits();
                    let exponent = ((bits >> M) & EXP_MASK) - EXP_OFFSET;
                    let mantissa = (bits & MAN_MASK) | MAN_ONE;
                    if exponent <= 52 {
                        todo!("I256::from(mantissa) >> (52 - exponent)")
                    } else if exponent >= 255 {
                        if todo!("signum > 0") {
                            I256::MAX
                        } else {
                            I256::MIN
                        }
                    } else {
                        todo!("I256::from(mantissa) << (exponent - 52)")
                    }
                } else {
                    I256::ZERO
                }
            }
        }
    )*};
}

impl_as_i256_float! {
    f32[u32], f64[u64],
}

macro_rules! impl_try_into {
    ($($t:ty),* $(,)?) => {$(
        impl TryFrom<I256> for $t {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(x: I256) -> Result<Self, Self::Error> {
                if todo!("x <= <$t>::MAX.as_i256()") {
                    Ok(*x.low() as _)
                } else {
                    Err(tfie())
                }
            }
        }
    )*};
}

impl_try_into! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
    isize, usize,
}

macro_rules! impl_into_float {
    ($($t:ty => $f:ident),* $(,)?) => {$(
        impl From<I256> for $t {
            #[inline]
            fn from(x: I256) -> $t {
                x.$f()
            }
        }
    )*};
}

impl_into_float! {
    f32 => as_f32, f64 => as_f64,
}
