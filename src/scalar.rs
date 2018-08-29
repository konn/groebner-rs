use num_traits::*;
use ring::*;
use std::ops::{Add, Mul, Neg, Sub};

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

impl<T: Semiring> Semiring for Scalar<T> {
    fn from_nat(n: usize) -> Self {
        Scalar(<T as Semiring>::from_nat(n))
    }
}

impl<T: Ring> Ring for Scalar<T> {
    fn from_int(n: isize) -> Self {
        Scalar(T::from_int(n))
    }
}
