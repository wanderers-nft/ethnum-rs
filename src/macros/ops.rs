//! Module containing macros for implementing `core::ops` traits.

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
        __impl_ops_binop! {
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

        __impl_ops_divmod! {
            impl Div for $int {
                div => $div3; "divide by zero"
            }

            impl Rem for $int {
                rem => $rem3; "calculate the remainder with a divisor of zero"
            }
        }

        __impl_ops_binop_extra_variants! {
            impl Add for $int | $prim { add = + }
            impl Mul for $int | $prim { mul = * }
            impl Sub for $int | $prim { sub = - }

            impl Div for $int | $prim { div = / }
            impl Rem for $int | $prim { rem = % }
        }
    };
}

macro_rules! __impl_ops_binop {
    ($(
        impl $op:ident for $int:ident {
            $method:ident => $op3:path, $opc:path; $msg:expr
        }
    )*) => {$(
        impl ::core::ops::$op for &'_ $int {
            type Output = $int;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                let mut result = ::core::mem::MaybeUninit::uninit();
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

macro_rules! __impl_ops_divmod {
    ($(
        impl $op:ident for $int:ident {
            $method:ident => $op3:path; $msg:expr
        }
    )*) => {$(
        impl ::core::ops::$op for &'_ $int {
            type Output = $int;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                if *rhs == 0 {
                    panic!(concat!("attempt to ", $msg));
                }

                let mut result = ::core::mem::MaybeUninit::uninit();
                $op3(&mut result, self, rhs);
                unsafe { result.assume_init() }
            }
        }
    )*};
}

macro_rules! __impl_ops_binop_extra_variants {
    ($(
        impl $op:ident for $int:ident | $prim:ident { $method:ident = $x:tt }
    )*) => {$(
        __impl_ops_binop_ref! {
            impl $op for $int {
                $method(a: &'_  $int, b:      $int) {  a $x &b };
                $method(a:      $int, b: &'_  $int) { &a $x  b };
                $method(a:      $int, b:      $int) { &a $x &b };

                $method(a: &'_  $int, b:     $prim) { a $x $int::new(b) };
                $method(a: &'_  $int, b: &'_ $prim) {  a $x *b };
                $method(a:      $int, b: &'_ $prim) { &a $x *b };
                $method(a:      $int, b:     $prim) { &a $x  b };

                $method(a:     $prim, b: &'_  $int) { $int::new(a) $x b };
                $method(a: &'_ $prim, b: &'_  $int) { *a $x  b };
                $method(a: &'_ $prim, b:      $int) { *a $x &b };
                $method(a:     $prim, b:      $int) {  a $x &b };
            }
        }
    )*};
}

macro_rules! __impl_ops_binop_ref {
    ($(
        impl $op:ident for $int:ident {$(
          $method:ident($lhs:ident: $lhst:ty, $rhs:ident: $rhst:ty) $impl:block;
        )*}
    )*) => {$($(
        impl ::core::ops::$op<$rhst> for $lhst {
            type Output = $int;
            fn $method(self, rhs: $rhst) -> Self::Output {
                let ($lhs, $rhs) = (self, rhs);
                $impl
            }
        }
    )*)*}
}
