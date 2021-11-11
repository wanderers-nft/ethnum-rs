mod ops {
    #![doc = " Module `core::ops` trait implementations."]
    #![doc = ""]
    #![doc = " Trait implementations for `i128` are also provided to allow notation such"]
    #![doc = " as:"]
    #![doc = ""]
    #![doc = " ```"]
    #![doc = " # use ethnum::I256;"]
    #![doc = " let a = 1 + I256::ONE;"]
    #![doc = " let b = I256::ONE + 1;"]
    #![doc = " dbg!(a, b);"]
    #![doc = " ```"]
    use super::I256;
    use crate::intrinsics::*;
    use core::mem::MaybeUninit;
    #[doc = " A wrapper around `add2` that uses `I256`."]
    fn iadd2(_: &mut I256, _: &I256) {
        ::core::panicking::panic("not yet implemented")
    }
    #[doc = " A wrapper around `ashl2` that uses `I256`."]
    fn _ishl2(_: &mut I256, _: u32) {
        ::core::panicking::panic("not yet implemented")
    }
    #[doc = " A wrapper around `ashl3` that uses `I256`."]
    fn ishl3(_: &mut MaybeUninit<I256>, _: &I256, _: u32) {
        ::core::panicking::panic("not yet implemented")
    }
    impl ::core::ops::Add for &'_ I256 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            let mut result = ::core::mem::MaybeUninit::uninit();
            #[cfg(debug_assertions)]
            {
                if iaddc(&mut result, self, rhs) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["attempt to add with overflow"],
                        &match () {
                            _args => [],
                        },
                    ));
                }
            }
            unsafe { result.assume_init() }
        }
    }
    impl ::core::ops::Add<I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a + &b
            }
        }
    }
    impl ::core::ops::Add<&'_ I256> for I256 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a + b
            }
        }
    }
    impl ::core::ops::Add<I256> for I256 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a + &b
            }
        }
    }
    impl ::core::ops::Add<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a + I256::new(b)
            }
        }
    }
    impl ::core::ops::Add<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a + *b
            }
        }
    }
    impl ::core::ops::Add<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a + *b
            }
        }
    }
    impl ::core::ops::Add<i128> for I256 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a + b
            }
        }
    }
    impl ::core::ops::Add<&'_ I256> for i128 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                I256::new(a) + b
            }
        }
    }
    impl ::core::ops::Add<&'_ I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a + b
            }
        }
    }
    impl ::core::ops::Add<I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a + &b
            }
        }
    }
    impl ::core::ops::Add<I256> for i128 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a + &b
            }
        }
    }
    impl ::core::ops::Mul for &'_ I256 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            let mut result = ::core::mem::MaybeUninit::uninit();
            #[cfg(debug_assertions)]
            {
                if imulc(&mut result, self, rhs) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["attempt to multiply with overflow"],
                        &match () {
                            _args => [],
                        },
                    ));
                }
            }
            unsafe { result.assume_init() }
        }
    }
    impl ::core::ops::Mul<I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a * &b
            }
        }
    }
    impl ::core::ops::Mul<&'_ I256> for I256 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a * b
            }
        }
    }
    impl ::core::ops::Mul<I256> for I256 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a * &b
            }
        }
    }
    impl ::core::ops::Mul<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a * I256::new(b)
            }
        }
    }
    impl ::core::ops::Mul<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a * *b
            }
        }
    }
    impl ::core::ops::Mul<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a * *b
            }
        }
    }
    impl ::core::ops::Mul<i128> for I256 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a * b
            }
        }
    }
    impl ::core::ops::Mul<&'_ I256> for i128 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                I256::new(a) * b
            }
        }
    }
    impl ::core::ops::Mul<&'_ I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a * b
            }
        }
    }
    impl ::core::ops::Mul<I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a * &b
            }
        }
    }
    impl ::core::ops::Mul<I256> for i128 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a * &b
            }
        }
    }
    impl ::core::ops::Sub for &'_ I256 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            let mut result = ::core::mem::MaybeUninit::uninit();
            #[cfg(debug_assertions)]
            {
                if isubc(&mut result, self, rhs) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["attempt to subtract with overflow"],
                        &match () {
                            _args => [],
                        },
                    ));
                }
            }
            unsafe { result.assume_init() }
        }
    }
    impl ::core::ops::Sub<I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a - &b
            }
        }
    }
    impl ::core::ops::Sub<&'_ I256> for I256 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a - b
            }
        }
    }
    impl ::core::ops::Sub<I256> for I256 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a - &b
            }
        }
    }
    impl ::core::ops::Sub<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a - I256::new(b)
            }
        }
    }
    impl ::core::ops::Sub<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a - *b
            }
        }
    }
    impl ::core::ops::Sub<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a - *b
            }
        }
    }
    impl ::core::ops::Sub<i128> for I256 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a - b
            }
        }
    }
    impl ::core::ops::Sub<&'_ I256> for i128 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                I256::new(a) - b
            }
        }
    }
    impl ::core::ops::Sub<&'_ I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a - b
            }
        }
    }
    impl ::core::ops::Sub<I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a - &b
            }
        }
    }
    impl ::core::ops::Sub<I256> for i128 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a - &b
            }
        }
    }
    impl ::core::ops::Div for &'_ I256 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: Self) -> Self::Output {
            if *rhs == 0 {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["attempt to divide by zero"],
                    &match () {
                        _args => [],
                    },
                ));
            }
            let mut result = ::core::mem::MaybeUninit::uninit();
            idiv3(&mut result, self, rhs);
            unsafe { result.assume_init() }
        }
    }
    impl ::core::ops::Div<I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a / &b
            }
        }
    }
    impl ::core::ops::Div<&'_ I256> for I256 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a / b
            }
        }
    }
    impl ::core::ops::Div<I256> for I256 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a / &b
            }
        }
    }
    impl ::core::ops::Div<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a / I256::new(b)
            }
        }
    }
    impl ::core::ops::Div<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a / *b
            }
        }
    }
    impl ::core::ops::Div<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a / *b
            }
        }
    }
    impl ::core::ops::Div<i128> for I256 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a / b
            }
        }
    }
    impl ::core::ops::Div<&'_ I256> for i128 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                I256::new(a) / b
            }
        }
    }
    impl ::core::ops::Div<&'_ I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a / b
            }
        }
    }
    impl ::core::ops::Div<I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a / &b
            }
        }
    }
    impl ::core::ops::Div<I256> for i128 {
        type Output = I256;
        #[inline]
        fn div(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a / &b
            }
        }
    }
    impl ::core::ops::Rem for &'_ I256 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: Self) -> Self::Output {
            if *rhs == 0 {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["attempt to calculate the remainder with a divisor of zero"],
                    &match () {
                        _args => [],
                    },
                ));
            }
            let mut result = ::core::mem::MaybeUninit::uninit();
            irem3(&mut result, self, rhs);
            unsafe { result.assume_init() }
        }
    }
    impl ::core::ops::Rem<I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a % &b
            }
        }
    }
    impl ::core::ops::Rem<&'_ I256> for I256 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a % b
            }
        }
    }
    impl ::core::ops::Rem<I256> for I256 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a % &b
            }
        }
    }
    impl ::core::ops::Rem<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a % I256::new(b)
            }
        }
    }
    impl ::core::ops::Rem<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a % *b
            }
        }
    }
    impl ::core::ops::Rem<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a % *b
            }
        }
    }
    impl ::core::ops::Rem<i128> for I256 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a % b
            }
        }
    }
    impl ::core::ops::Rem<&'_ I256> for i128 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                I256::new(a) % b
            }
        }
    }
    impl ::core::ops::Rem<&'_ I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a % b
            }
        }
    }
    impl ::core::ops::Rem<I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a % &b
            }
        }
    }
    impl ::core::ops::Rem<I256> for i128 {
        type Output = I256;
        #[inline]
        fn rem(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a % &b
            }
        }
    }
    impl ::core::ops::Shl<u32> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u32) -> Self::Output {
            #[cfg(debug_assertions)]
            if rhs > 0xff {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["attempt to shift left with overflow"],
                    &match () {
                        _args => [],
                    },
                ));
            }
            let mut result = ::core::mem::MaybeUninit::uninit();
            ishl3(&mut result, self, rhs);
            unsafe { result.assume_init() }
        }
    }
    impl ::core::ops::Shl<&'_ u32> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ u32> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<u32> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<crate::int::I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: crate::int::I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ crate::int::I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ crate::int::I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ crate::int::I256> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ crate::int::I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<crate::int::I256> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: crate::int::I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<crate::uint::U256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: crate::uint::U256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ crate::uint::U256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ crate::uint::U256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ crate::uint::U256> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ crate::uint::U256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<crate::uint::U256> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: crate::uint::U256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<i8> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i8> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i8> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<i8> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<i16> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i16> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i16> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<i16> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<i32> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i32> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i32> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<i32> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<i64> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i64> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i64> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<i64> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<i128> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<isize> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: isize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ isize> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ isize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ isize> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ isize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<isize> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: isize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<u8> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ u8> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ u8> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<u8> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<u16> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ u16> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ u16> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<u16> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<u64> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ u64> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ u64> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<u64> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<u128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ u128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ u128> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ u128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<u128> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: u128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shl<usize> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: usize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a << b
            }
        }
    }
    impl ::core::ops::Shl<&'_ usize> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ usize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a << *b
            }
        }
    }
    impl ::core::ops::Shl<&'_ usize> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: &'_ usize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << *b
            }
        }
    }
    impl ::core::ops::Shl<usize> for I256 {
        type Output = I256;
        #[inline]
        fn shl(self, rhs: usize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a << b
            }
        }
    }
    impl ::core::ops::Shr<u32> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u32) -> Self::Output {
            #[cfg(debug_assertions)]
            if rhs > 0xff {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["attempt to shift right with overflow"],
                    &match () {
                        _args => [],
                    },
                ));
            }
            let mut result = ::core::mem::MaybeUninit::uninit();
            ashr3(&mut result, self, rhs);
            unsafe { result.assume_init() }
        }
    }
    impl ::core::ops::Shr<&'_ u32> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ u32> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<u32> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<crate::int::I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: crate::int::I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ crate::int::I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ crate::int::I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ crate::int::I256> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ crate::int::I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<crate::int::I256> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: crate::int::I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<crate::uint::U256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: crate::uint::U256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ crate::uint::U256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ crate::uint::U256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ crate::uint::U256> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ crate::uint::U256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<crate::uint::U256> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: crate::uint::U256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<i8> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i8> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i8> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<i8> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<i16> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i16> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i16> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<i16> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<i32> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i32> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i32> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<i32> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i32) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<i64> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i64> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i64> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<i64> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<i128> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<isize> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: isize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ isize> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ isize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ isize> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ isize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<isize> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: isize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<u8> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ u8> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ u8> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<u8> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u8) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<u16> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ u16> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ u16> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<u16> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u16) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<u64> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ u64> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ u64> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<u64> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u64) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<u128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ u128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ u128> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ u128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<u128> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: u128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Shr<usize> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: usize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                #[cfg(debug_assertions)]
                let b = u32::try_from(b).unwrap_or(u32::MAX);
                a >> b
            }
        }
    }
    impl ::core::ops::Shr<&'_ usize> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ usize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a >> *b
            }
        }
    }
    impl ::core::ops::Shr<&'_ usize> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: &'_ usize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> *b
            }
        }
    }
    impl ::core::ops::Shr<usize> for I256 {
        type Output = I256;
        #[inline]
        fn shr(self, rhs: usize) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a >> b
            }
        }
    }
    impl ::core::ops::Not for I256 {
        type Output = I256;
        #[inline]
        fn not(self) -> Self::Output {
            let x = self;
            {
                let I256([a, b]) = x;
                I256([!a, !b])
            }
        }
    }
    impl ::core::ops::Not for &'_ I256 {
        type Output = I256;
        #[inline]
        fn not(self) -> Self::Output {
            let x = self;
            {
                let I256([a, b]) = x;
                I256([!a, !b])
            }
        }
    }
    impl ::core::ops::BitAnd<&'_ I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: &'_ I256) -> Self::Output {
            let I256([a0, a1]) = self;
            let I256([b0, b1]) = rhs;
            I256([a0 & b0, a1 & b1])
        }
    }
    impl ::core::ops::BitAnd<I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a & &b
            }
        }
    }
    impl ::core::ops::BitAnd<&'_ I256> for I256 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a & b
            }
        }
    }
    impl ::core::ops::BitAnd<I256> for I256 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a & &b
            }
        }
    }
    impl ::core::ops::BitAnd<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a & I256::new(b)
            }
        }
    }
    impl ::core::ops::BitAnd<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a & *b
            }
        }
    }
    impl ::core::ops::BitAnd<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a & *b
            }
        }
    }
    impl ::core::ops::BitAnd<i128> for I256 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a & b
            }
        }
    }
    impl ::core::ops::BitAnd<&'_ I256> for i128 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                I256::new(a) & b
            }
        }
    }
    impl ::core::ops::BitAnd<&'_ I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a & b
            }
        }
    }
    impl ::core::ops::BitAnd<I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a & &b
            }
        }
    }
    impl ::core::ops::BitAnd<I256> for i128 {
        type Output = I256;
        #[inline]
        fn bitand(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a & &b
            }
        }
    }
    impl ::core::ops::BitOr<&'_ I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: &'_ I256) -> Self::Output {
            let I256([a0, a1]) = self;
            let I256([b0, b1]) = rhs;
            I256([a0 | b0, a1 | b1])
        }
    }
    impl ::core::ops::BitOr<I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a | &b
            }
        }
    }
    impl ::core::ops::BitOr<&'_ I256> for I256 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a | b
            }
        }
    }
    impl ::core::ops::BitOr<I256> for I256 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a | &b
            }
        }
    }
    impl ::core::ops::BitOr<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a | I256::new(b)
            }
        }
    }
    impl ::core::ops::BitOr<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a | *b
            }
        }
    }
    impl ::core::ops::BitOr<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a | *b
            }
        }
    }
    impl ::core::ops::BitOr<i128> for I256 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a | b
            }
        }
    }
    impl ::core::ops::BitOr<&'_ I256> for i128 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                I256::new(a) | b
            }
        }
    }
    impl ::core::ops::BitOr<&'_ I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a | b
            }
        }
    }
    impl ::core::ops::BitOr<I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a | &b
            }
        }
    }
    impl ::core::ops::BitOr<I256> for i128 {
        type Output = I256;
        #[inline]
        fn bitor(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a | &b
            }
        }
    }
    impl ::core::ops::BitXor<&'_ I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: &'_ I256) -> Self::Output {
            let I256([a0, a1]) = self;
            let I256([b0, b1]) = rhs;
            I256([a0 ^ b0, a1 ^ b1])
        }
    }
    impl ::core::ops::BitXor<I256> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a ^ &b
            }
        }
    }
    impl ::core::ops::BitXor<&'_ I256> for I256 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a ^ b
            }
        }
    }
    impl ::core::ops::BitXor<I256> for I256 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a ^ &b
            }
        }
    }
    impl ::core::ops::BitXor<i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a ^ I256::new(b)
            }
        }
    }
    impl ::core::ops::BitXor<&'_ i128> for &'_ I256 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a ^ *b
            }
        }
    }
    impl ::core::ops::BitXor<&'_ i128> for I256 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: &'_ i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a ^ *b
            }
        }
    }
    impl ::core::ops::BitXor<i128> for I256 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: i128) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                &a ^ b
            }
        }
    }
    impl ::core::ops::BitXor<&'_ I256> for i128 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                I256::new(a) ^ b
            }
        }
    }
    impl ::core::ops::BitXor<&'_ I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: &'_ I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a ^ b
            }
        }
    }
    impl ::core::ops::BitXor<I256> for &'_ i128 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                *a ^ &b
            }
        }
    }
    impl ::core::ops::BitXor<I256> for i128 {
        type Output = I256;
        #[inline]
        fn bitxor(self, rhs: I256) -> Self::Output {
            let (a, b) = (self, rhs);
            {
                a ^ &b
            }
        }
    }
    impl ::core::ops::Neg for I256 {
        type Output = I256;
        #[inline]
        fn neg(self) -> Self::Output {
            let x = self;
            {
                let mut x = !x;
                iadd2(&mut x, &I256::ONE);
                x
            }
        }
    }
    impl ::core::ops::Neg for &'_ I256 {
        type Output = I256;
        #[inline]
        fn neg(self) -> Self::Output {
            let x = self;
            {
                let mut x = !x;
                iadd2(&mut x, &I256::ONE);
                x
            }
        }
    }
}
