mod ops {
    #![doc = " Module containing macros for implementing `core::ops` traits."]
    use crate::{int::I256, intrinsics::*};
    use core::{mem::MaybeUninit, ops};
    pub(crate) use impl_ops;
    impl ops::Add for &'_ I256 {
        type Output = I256;
        #[inline]
        fn add(self, rhs: &'_ I256) -> Self::Output {
            let mut result = MaybeUninit::uninit();
            #[cfg(debug_assertions)]
            {
                if iaddc(&mut result, self, rhs) {
                    ::core::panicking::panic_str("attempt to add with overflow");
                }
            }
            unsafe { result.assume_init() }
        }
    }
    impl ops::Mul for &'_ I256 {
        type Output = I256;
        #[inline]
        fn mul(self, rhs: &'_ I256) -> Self::Output {
            let mut result = MaybeUninit::uninit();
            #[cfg(debug_assertions)]
            {
                if imulc(&mut result, self, rhs) {
                    ::core::panicking::panic_str("attempt to multiply with overflow");
                }
            }
            unsafe { result.assume_init() }
        }
    }
    impl ops::Sub for &'_ I256 {
        type Output = I256;
        #[inline]
        fn sub(self, rhs: &'_ I256) -> Self::Output {
            let mut result = MaybeUninit::uninit();
            #[cfg(debug_assertions)]
            {
                if isubc(&mut result, self, rhs) {
                    ::core::panicking::panic_str("attempt to subtract with overflow");
                }
            }
            unsafe { result.assume_init() }
        }
    }
    impl ops::Add<I256> for &'_ I256 {
        type Output = I256;
        fn add(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a + &b
            }
        }
    }
    impl ops::Add<&'_ I256> for I256 {
        type Output = I256;
        fn add(self, rhs: &'_ I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a + b
            }
        }
    }
    impl ops::Add<I256> for I256 {
        type Output = I256;
        fn add(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a + &b
            }
        }
    }
    impl ops::Add<i128> for &'_ I256 {
        type Output = I256;
        fn add(self, rhs: i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a + &I256::new(b)
            }
        }
    }
    impl ops::Add<&'_ i128> for &'_ I256 {
        type Output = I256;
        fn add(self, rhs: &'_ i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a + *b
            }
        }
    }
    impl ops::Add<&'_ i128> for I256 {
        type Output = I256;
        fn add(self, rhs: &'_ i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a + *b
            }
        }
    }
    impl ops::Add<i128> for I256 {
        type Output = I256;
        fn add(self, rhs: i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a + b
            }
        }
    }
    impl ops::Add<&'_ I256> for i128 {
        type Output = I256;
        fn add(self, rhs: &'_ I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &I256::new(a) + b
            }
        }
    }
    impl ops::Add<&'_ I256> for &'_ i128 {
        type Output = I256;
        fn add(self, rhs: &'_ I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                *a + b
            }
        }
    }
    impl ops::Add<I256> for &'_ i128 {
        type Output = I256;
        fn add(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                *a + &b
            }
        }
    }
    impl ops::Add<I256> for i128 {
        type Output = I256;
        fn add(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a + &b
            }
        }
    }
    impl ops::Mul<I256> for &'_ I256 {
        type Output = I256;
        fn mul(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a * &b
            }
        }
    }
    impl ops::Mul<&'_ I256> for I256 {
        type Output = I256;
        fn mul(self, rhs: &'_ I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a * b
            }
        }
    }
    impl ops::Mul<I256> for I256 {
        type Output = I256;
        fn mul(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a * &b
            }
        }
    }
    impl ops::Mul<i128> for &'_ I256 {
        type Output = I256;
        fn mul(self, rhs: i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a * &I256::new(b)
            }
        }
    }
    impl ops::Mul<&'_ i128> for &'_ I256 {
        type Output = I256;
        fn mul(self, rhs: &'_ i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a * *b
            }
        }
    }
    impl ops::Mul<&'_ i128> for I256 {
        type Output = I256;
        fn mul(self, rhs: &'_ i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a * *b
            }
        }
    }
    impl ops::Mul<i128> for I256 {
        type Output = I256;
        fn mul(self, rhs: i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a * b
            }
        }
    }
    impl ops::Mul<&'_ I256> for i128 {
        type Output = I256;
        fn mul(self, rhs: &'_ I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &I256::new(a) * b
            }
        }
    }
    impl ops::Mul<&'_ I256> for &'_ i128 {
        type Output = I256;
        fn mul(self, rhs: &'_ I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                *a * b
            }
        }
    }
    impl ops::Mul<I256> for &'_ i128 {
        type Output = I256;
        fn mul(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                *a * &b
            }
        }
    }
    impl ops::Mul<I256> for i128 {
        type Output = I256;
        fn mul(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a * &b
            }
        }
    }
    impl ops::Sub<I256> for &'_ I256 {
        type Output = I256;
        fn sub(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a - &b
            }
        }
    }
    impl ops::Sub<&'_ I256> for I256 {
        type Output = I256;
        fn sub(self, rhs: &'_ I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a - b
            }
        }
    }
    impl ops::Sub<I256> for I256 {
        type Output = I256;
        fn sub(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a - &b
            }
        }
    }
    impl ops::Sub<i128> for &'_ I256 {
        type Output = I256;
        fn sub(self, rhs: i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a - &I256::new(b)
            }
        }
    }
    impl ops::Sub<&'_ i128> for &'_ I256 {
        type Output = I256;
        fn sub(self, rhs: &'_ i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a - *b
            }
        }
    }
    impl ops::Sub<&'_ i128> for I256 {
        type Output = I256;
        fn sub(self, rhs: &'_ i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a - *b
            }
        }
    }
    impl ops::Sub<i128> for I256 {
        type Output = I256;
        fn sub(self, rhs: i128) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &a - b
            }
        }
    }
    impl ops::Sub<&'_ I256> for i128 {
        type Output = I256;
        fn sub(self, rhs: &'_ I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                &I256::new(a) - b
            }
        }
    }
    impl ops::Sub<&'_ I256> for &'_ i128 {
        type Output = I256;
        fn sub(self, rhs: &'_ I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                *a - b
            }
        }
    }
    impl ops::Sub<I256> for &'_ i128 {
        type Output = I256;
        fn sub(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                *a - &b
            }
        }
    }
    impl ops::Sub<I256> for i128 {
        type Output = I256;
        fn sub(self, rhs: I256) -> Self::Output {
            let a = self;
            let b = rhs;
            {
                a - &b
            }
        }
    }
}
