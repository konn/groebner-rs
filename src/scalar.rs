use num_traits::*;
use ring::*;
use std::ops::*;

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub struct Scalar<T>(pub T);

impl<T: Add<Output = T>> Add<T> for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, t: T) -> Scalar<T> {
        Scalar(self.0 + t)
    }
}

impl<R: Zero> Zero for Scalar<R> {
    fn zero() -> Scalar<R> {
        Scalar(R::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<R: One> One for Scalar<R> {
    fn one() -> Scalar<R> {
        Scalar(R::one())
    }
}

impl<T: Add<Output = T>> Add for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, t: Scalar<T>) -> Scalar<T> {
        Scalar(self.0 + t.0)
    }
}

impl<T: Mul<Output = T>> Mul for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, t: Scalar<T>) -> Scalar<T> {
        Scalar(self.0 * t.0)
    }
}

impl<T: Mul<Output = T>> Mul<T> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, t: T) -> Scalar<T> {
        Scalar(self.0 * t)
    }
}

impl<T: Neg<Output = T>> Neg for Scalar<T> {
    type Output = Scalar<T>;
    fn neg(self) -> Scalar<T> {
        Scalar(self.0.neg())
    }
}

impl<T: Sub<Output = T>> Sub for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, other: Scalar<T>) -> Scalar<T> {
        Scalar(self.0 - other.0)
    }
}

impl<T: Ring> Semiring for Scalar<T> {
    fn from_nat(n: usize) -> Self {
        Scalar(<T as Semiring>::from_nat(n))
    }
}

impl<T: Div<T, Output = T>> Div for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, otr: Scalar<T>) -> Scalar<T> {
        Scalar(self.0 / otr.0)
    }
}

impl<T: Ring> Ring for Scalar<T> {
    fn from_int(n: isize) -> Self {
        Scalar(T::from_int(n))
    }
}

impl<T: Field> Field for Scalar<T> {}

derive_assign_with!(impl AddAssign, add_assign as Add, add for Scalar<T> where T: Ring);
derive_assign_with!(impl SubAssign, sub_assign as Sub, sub for Scalar<T> where T: Ring);
derive_assign_with!(impl MulAssign, mul_assign as Mul, mul for Scalar<T> where T: Ring);
derive_assign_with!(impl DivAssign, div_assign as Div, div for Scalar<T> where T: Field);
