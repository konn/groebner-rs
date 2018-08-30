use num_integer::*;
use num_rational::*;
use num_traits::*;
use std::iter;
use std::ops::*;

pub trait Semiring: Clone + Eq + Zero + One + AddAssign<Self> + MulAssign<Self> {
    fn from_nat(i: usize) -> Self {
        iter::repeat(Self::one())
            .take(i)
            .fold(Self::zero(), |a, b| a + b)
    }

    fn pow(self, n: usize) -> Self {
        pow::pow(self, n)
    }
}

pub trait Ring: Semiring + Sub<Output = Self> + Neg<Output = Self> + SubAssign<Self> {
    fn from_int(n: isize) -> Self {
        if n < 0 {
            -Self::from_nat(n.abs() as usize)
        } else {
            Self::from_nat(n as usize)
        }
    }
}

impl Semiring for usize {
    fn from_nat(n: usize) -> usize {
        n
    }
}

impl Semiring for isize {
    fn from_nat(i: usize) -> isize {
        i as isize
    }
}

impl Ring for isize {
    fn from_int(i: isize) -> isize {
        i
    }
}

pub trait Field: Ring + Div<Self, Output = Self> {
    fn try_div(self, other: Self) -> Option<Self> {
        if other.is_zero() {
            None
        } else {
            Some(self / other)
        }
    }

    fn recip(self) -> Self {
        Self::one() / self
    }
}

impl<I: NumAssign + Ring + Integer> Semiring for Ratio<I> {}

impl<I: NumAssign + Ring + Integer> Ring for Ratio<I> {}
impl<I: NumAssign + Ring + Integer> Field for Ratio<I> {}
