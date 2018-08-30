#![allow(unknown_lints)]
#![allow(unused_macros)]

macro_rules! lift_binop_to_ref {
    (impl $imp:ident, $method:ident for $t:ty where $($bd_type:ident : $bound:tt),*) => {
        impl<'a, 'b, $($bd_type: $bound,)*> $imp<&'b $t> for &'a $t {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: &$t) -> Self::Output {
                $imp::$method(self.clone(), other.clone())
            }
        }
        impl<'a, $($bd_type: $bound,)*> $imp<&'a $t> for $t {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: &$t) -> Self::Output {
                $imp::$method(self, other.clone())
            }
        }
        impl<'a, $($bd_type: $bound,)*> $imp<$t> for &'a $t {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: $t) -> Self::Output {
                $imp::$method(self.clone(), other)
            }
        }
    };
}

macro_rules! lift_unaop_to_ref {
    (impl $imp:ident, $method:ident for $t:ty where $($bd_type:ident : $bound:tt),*) => {
        impl<'a, $($bd_type: $bound,)*> $imp for &'a $t {
            type Output = <$t as $imp>::Output;

            fn $method(self) -> Self::Output {
                $imp::$method(self.clone())
            }
        }
    };
}

macro_rules! derive_assign_with {
    (
        impl
        $imp:ident,
        $method:ident as
        $cls:ident,
        $op:ident for
        $t:ty where
        $($bd_type:ident :
         $bound:tt),*
    ) => {
        impl<$($bd_type: $bound,)*> $imp<$t> for $t {
            fn $method(&mut self, other: $t) {
                *self = $cls::$op(self.clone(), other);
            }
        }
    };
}

macro_rules! lift_nums_to_ref {
    (impl for $t:ty where $($bd_type:ident : $bound:tt),*) => {
        lift_binop_to_ref!(impl Add, add for $t where $($bd_type : $bound),*);
        derive_assign_with!(impl AddAssign, add_assign as Add, add for $t where $($bd_type : $bound),*);
        lift_binop_to_ref!(impl Mul, mul for $t where $($bd_type : $bound),*);
        derive_assign_with!(impl MulAssign, mul_assign as Mul, mul for $t where $($bd_type : $bound),*);
        lift_binop_to_ref!(impl Sub, sub for $t where $($bd_type : $bound),*);
        derive_assign_with!(impl SubAssign, sub_assign as Sub, sub for $t where $($bd_type : $bound),*);
        lift_unaop_to_ref!(impl Neg, neg for $t where $($bd_type : $bound),*);
    };
}
