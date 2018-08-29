use monomial::*;
use num_traits::*;
use polynomial::Polynomial;
use ring::*;
use scalar::*;
use std::collections::BTreeMap;
use std::ops::{Add, Mul, Neg, Sub};

use std::iter;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unipol<R> {
    coeffs: Vec<R>,
}

impl<R: Ring> Unipol<R> {
    pub fn x() -> Unipol<R> {
        Unipol {
            coeffs: vec![R::zero(), R::one()],
        }
    }
}

impl<R: Zero + Clone> Add for Unipol<R> {
    type Output = Unipol<R>;
    fn add(self, other: Self) -> Self {
        let len = self.coeffs.len().max(other.coeffs.len());
        let x_pad_len = len - self.coeffs.len();
        let x_pad = self
            .coeffs
            .into_iter()
            .chain(iter::repeat(R::zero()).take(x_pad_len));
        let y_pad_len = len - other.coeffs.len();
        let y_pad = other
            .coeffs
            .into_iter()
            .chain(iter::repeat(R::zero()).take(y_pad_len));
        let coeffs = x_pad.zip(y_pad).map(|(a, b)| a + b).collect();
        Unipol { coeffs }
    }
}

impl<R: One + Zero + Clone> Mul<Unipol<R>> for Scalar<R> {
    type Output = Unipol<R>;
    fn mul(self, Unipol { coeffs }: Unipol<R>) -> Unipol<R> {
        if self.0.is_zero() {
            Unipol { coeffs: vec![] }
        } else {
            Unipol {
                coeffs: coeffs.into_iter().map(|r| self.0.clone() * r).collect(),
            }
        }
    }
}

impl<R: Ring> Neg for Unipol<R> {
    type Output = Unipol<R>;
    fn neg(self) -> Self {
        Unipol {
            coeffs: self.coeffs.into_iter().map(|a| a.neg()).collect(),
        }
    }
}

impl<R: Ring> Sub for Unipol<R> {
    type Output = Unipol<R>;
    fn sub(self, other: Unipol<R>) -> Self {
        self + other.neg()
    }
}

impl<R: Semiring> Mul for Unipol<R> {
    type Output = Unipol<R>;
    fn mul(self, Unipol { coeffs: rs }: Unipol<R>) -> Unipol<R> {
        let Unipol { coeffs: ls } = self;

        ls.into_iter()
            .enumerate()
            .map(|(i, c)| Unipol {
                coeffs: iter::repeat(R::zero())
                    .take(i)
                    .chain(rs.iter().cloned().map(|k| c.clone() * k))
                    .collect(),
            })
            .fold(Unipol { coeffs: vec![] }, Unipol::add)
    }
}

impl<R: Zero + Clone> Zero for Unipol<R> {
    fn zero() -> Self {
        Unipol { coeffs: vec![] }
    }

    fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }
}

impl<R: Semiring> One for Unipol<R> {
    fn one() -> Self {
        Unipol {
            coeffs: vec![R::one()],
        }
    }
}

impl<R: Semiring> Semiring for Unipol<R> {
    fn from_nat(n: usize) -> Self {
        Unipol {
            coeffs: vec![R::from_nat(n)],
        }
    }
}

impl<R: Ring> Ring for Unipol<R> {
    fn from_int(n: isize) -> Self {
        Unipol {
            coeffs: vec![R::from_int(n)],
        }
    }
}

impl<'a, R: Ring + 'a> Polynomial<'a> for Unipol<R> {
    type Monomial = Power;
    type Coeff = R;

    fn lead_term(&'a self) -> Option<(Power, &'a R)> {
        self.coeffs
            .iter()
            .enumerate()
            .last()
            .map(|(a, b)| (Power(a), b))
    }

    fn var(_: ()) -> Option<Self> {
        Some(Unipol {
            coeffs: vec![R::one()],
        })
    }

    fn terms(&self) -> BTreeMap<Self::Monomial, &Self::Coeff> {
        self.coeffs
            .iter()
            .enumerate()
            .map(|(a, b)| (Power(a), b))
            .collect()
    }
}
