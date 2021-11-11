//! Module containing macros for implementing `core::ops` traits.

use crate::{int::I256, intrinsics::*};
use core::{mem::MaybeUninit, ops};

macro_rules! impl_ops {
    (
        for $int:ident ($prim:ident) {
            add => $add3:ident, $addc:ident;
            mul => $mul3:ident, $mulc:ident;
            sub => $sub3:ident, $subc:ident;
            div => $div3:ident;
            rem => $rem3:ident;
            shl => $shl3:ident;
            shr => $shr3:ident;
        }
    ) => {
        impl_binop! {
            impl Add for $int {
                add => $add3, $addc; "add with overflow"
            }

            impl Mul for $int {
                mul => $mul3, $mulc; "multiply with overflow"
            }

            impl Sub for $int {
                sub => $sub3, $subc; "subtract with overflow"
            }
        }
    };
}

macro_rules! impl_binop {
    ($(
        impl $op:ident for $int:ident {
            $method:ident => $op3:path, $opc:path; $msg:expr
        }
    )*) => {$(
        impl ops::$op for &'_ $int {
            type Output = $int;

            #[inline]
            fn $method(self, rhs: &'_ $int) -> Self::Output {
                let mut result = MaybeUninit::uninit();
                #[cfg(not(debug_assertions))]
                {
                    $op3(&mut result, self, rhs);
                }
                #[cfg(debug_assertions)]
                {
                    if $opc(&mut result, self, rhs) {
                        panic!(concat!("attempt to ", $msg));
                    }
                }
                unsafe { result.assume_init() }
            }
        }
    )*};
}

impl_ops! {
    for I256 (i128) {
        add => add3, iaddc;
        mul => imul3, imulc;
        sub => sub3, isubc;
        div => idiv3;
        rem => irem3;
        shl => ashl3;
        shr => ashr3;
    }
}

#[cfg(test)]
mod tests {
    use crate::U256;
    use core::ops::*;

    #[test]
    fn trait_implementations() {
        trait Implements {}
        impl Implements for U256 {}
        impl Implements for &'_ U256 {}

        fn assert_ops<T>()
        where
            for<'a> T: Implements
                + Add<&'a u128>
                + Add<&'a U256>
                + Add<u128>
                + Add<U256>
                + AddAssign<&'a u128>
                + AddAssign<&'a U256>
                + AddAssign<u128>
                + AddAssign<U256>
                + BitAnd<&'a u128>
                + BitAnd<&'a U256>
                + BitAnd<u128>
                + BitAnd<U256>
                + BitAndAssign<&'a u128>
                + BitAndAssign<&'a U256>
                + BitAndAssign<u128>
                + BitAndAssign<U256>
                + BitOr<&'a u128>
                + BitOr<&'a U256>
                + BitOr<u128>
                + BitOr<U256>
                + BitOrAssign<&'a u128>
                + BitOrAssign<&'a U256>
                + BitOrAssign<u128>
                + BitOrAssign<U256>
                + BitXor<&'a u128>
                + BitXor<&'a U256>
                + BitXor<u128>
                + BitXor<U256>
                + BitXorAssign<&'a u128>
                + BitXorAssign<&'a U256>
                + BitXorAssign<u128>
                + BitXorAssign<U256>
                + Div<&'a u128>
                + Div<&'a U256>
                + Div<u128>
                + Div<U256>
                + DivAssign<&'a u128>
                + DivAssign<&'a U256>
                + DivAssign<u128>
                + DivAssign<U256>
                + Mul<&'a u128>
                + Mul<&'a U256>
                + Mul<u128>
                + Mul<U256>
                + MulAssign<&'a u128>
                + MulAssign<&'a U256>
                + MulAssign<u128>
                + MulAssign<U256>
                + Not
                + Rem<&'a u128>
                + Rem<&'a U256>
                + Rem<u128>
                + Rem<U256>
                + RemAssign<&'a u128>
                + RemAssign<&'a U256>
                + RemAssign<u128>
                + RemAssign<U256>
                + Shl<&'a i128>
                + Shl<&'a i16>
                + Shl<&'a i32>
                + Shl<&'a i64>
                + Shl<&'a i8>
                + Shl<&'a isize>
                + Shl<&'a u128>
                + Shl<&'a u16>
                + Shl<&'a U256>
                + Shl<&'a u32>
                + Shl<&'a u64>
                + Shl<&'a u8>
                + Shl<&'a usize>
                + Shl<i128>
                + Shl<i16>
                + Shl<i32>
                + Shl<i64>
                + Shl<i8>
                + Shl<isize>
                + Shl<u128>
                + Shl<u16>
                + Shl<U256>
                + Shl<u32>
                + Shl<u64>
                + Shl<u8>
                + Shl<usize>
                + ShlAssign<&'a i128>
                + ShlAssign<&'a i16>
                + ShlAssign<&'a i32>
                + ShlAssign<&'a i64>
                + ShlAssign<&'a i8>
                + ShlAssign<&'a isize>
                + ShlAssign<&'a u128>
                + ShlAssign<&'a u16>
                + ShlAssign<&'a U256>
                + ShlAssign<&'a u32>
                + ShlAssign<&'a u64>
                + ShlAssign<&'a u8>
                + ShlAssign<&'a usize>
                + ShlAssign<i128>
                + ShlAssign<i16>
                + ShlAssign<i32>
                + ShlAssign<i64>
                + ShlAssign<i8>
                + ShlAssign<isize>
                + ShlAssign<u128>
                + ShlAssign<u16>
                + ShlAssign<U256>
                + ShlAssign<u32>
                + ShlAssign<u64>
                + ShlAssign<u8>
                + ShlAssign<usize>
                + Shr<&'a i128>
                + Shr<&'a i16>
                + Shr<&'a i32>
                + Shr<&'a i64>
                + Shr<&'a i8>
                + Shr<&'a isize>
                + Shr<&'a u128>
                + Shr<&'a u16>
                + Shr<&'a U256>
                + Shr<&'a u32>
                + Shr<&'a u64>
                + Shr<&'a u8>
                + Shr<&'a usize>
                + Shr<i128>
                + Shr<i16>
                + Shr<i32>
                + Shr<i64>
                + Shr<i8>
                + Shr<isize>
                + Shr<u128>
                + Shr<u16>
                + Shr<U256>
                + Shr<u32>
                + Shr<u64>
                + Shr<u8>
                + Shr<usize>
                + ShrAssign<&'a i128>
                + ShrAssign<&'a i16>
                + ShrAssign<&'a i32>
                + ShrAssign<&'a i64>
                + ShrAssign<&'a i8>
                + ShrAssign<&'a isize>
                + ShrAssign<&'a u128>
                + ShrAssign<&'a u16>
                + ShrAssign<&'a U256>
                + ShrAssign<&'a u32>
                + ShrAssign<&'a u64>
                + ShrAssign<&'a u8>
                + ShrAssign<&'a usize>
                + ShrAssign<i128>
                + ShrAssign<i16>
                + ShrAssign<i32>
                + ShrAssign<i64>
                + ShrAssign<i8>
                + ShrAssign<isize>
                + ShrAssign<u128>
                + ShrAssign<u16>
                + ShrAssign<U256>
                + ShrAssign<u32>
                + ShrAssign<u64>
                + ShrAssign<u8>
                + ShrAssign<usize>
                + Sub<&'a u128>
                + Sub<&'a U256>
                + Sub<u128>
                + Sub<U256>
                + SubAssign<&'a u128>
                + SubAssign<&'a U256>
                + SubAssign<u128>
                + SubAssign<U256>,
            for<'a> &'a T: Implements
                + Add<&'a u128>
                + Add<&'a U256>
                + Add<u128>
                + Add<U256>
                + BitAnd<&'a u128>
                + BitAnd<&'a U256>
                + BitAnd<u128>
                + BitAnd<U256>
                + BitOr<&'a u128>
                + BitOr<&'a U256>
                + BitOr<u128>
                + BitOr<U256>
                + BitXor<&'a u128>
                + BitXor<&'a U256>
                + BitXor<u128>
                + BitXor<U256>
                + Div<&'a u128>
                + Div<&'a U256>
                + Div<u128>
                + Div<U256>
                + Mul<&'a u128>
                + Mul<&'a U256>
                + Mul<u128>
                + Mul<U256>
                + Not
                + Rem<&'a u128>
                + Rem<&'a U256>
                + Rem<u128>
                + Rem<U256>
                + Shl<&'a i128>
                + Shl<&'a i16>
                + Shl<&'a i32>
                + Shl<&'a i64>
                + Shl<&'a i8>
                + Shl<&'a isize>
                + Shl<&'a u128>
                + Shl<&'a u16>
                + Shl<&'a U256>
                + Shl<&'a u32>
                + Shl<&'a u64>
                + Shl<&'a u8>
                + Shl<&'a usize>
                + Shl<i128>
                + Shl<i16>
                + Shl<i32>
                + Shl<i64>
                + Shl<i8>
                + Shl<isize>
                + Shl<u128>
                + Shl<u16>
                + Shl<U256>
                + Shl<u32>
                + Shl<u64>
                + Shl<u8>
                + Shl<usize>
                + Shr<&'a i128>
                + Shr<&'a i16>
                + Shr<&'a i32>
                + Shr<&'a i64>
                + Shr<&'a i8>
                + Shr<&'a isize>
                + Shr<&'a u128>
                + Shr<&'a u16>
                + Shr<&'a U256>
                + Shr<&'a u32>
                + Shr<&'a u64>
                + Shr<&'a u8>
                + Shr<&'a usize>
                + Shr<i128>
                + Shr<i16>
                + Shr<i32>
                + Shr<i64>
                + Shr<i8>
                + Shr<isize>
                + Shr<u128>
                + Shr<u16>
                + Shr<U256>
                + Shr<u32>
                + Shr<u64>
                + Shr<u8>
                + Shr<usize>
                + Sub<&'a u128>
                + Sub<&'a U256>
                + Sub<u128>
                + Sub<U256>,
        {
        }

        assert_ops::<U256>();
    }
}
