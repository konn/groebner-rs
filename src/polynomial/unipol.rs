use crate::monomial::*;
use crate::polynomial::Polynomial;
use crate::ring::*;
use crate::scalar::*;
use num_traits::*;
use std::collections::BTreeMap;
use std::iter;
use std::ops::*;
use std::slice;
use std::vec;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unipol<R> {
    coeffs: Vec<R>,
}

impl<R: Zero> Unipol<R> {
    fn normalise(self) -> Self {
        let Unipol { coeffs } = self;
        let mut coeffs: Vec<R> = coeffs.into_iter().rev().skip_while(Zero::is_zero).collect();
        coeffs.reverse();
        Unipol { coeffs }
    }
}

impl<R: One + Zero> Unipol<R> {
    pub fn x() -> Unipol<R> {
        Unipol {
            coeffs: vec![R::zero(), R::one()],
        }
    }
}

impl<R: Zero + Clone> Add for Unipol<R> {
    type Output = Unipol<R>;
    #[allow(clippy::suspicious_arithmetic_impl)]
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
        Unipol { coeffs }.normalise()
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
            .normalise()
        }
    }
}

impl<R: Ring> Neg for Unipol<R> {
    type Output = Unipol<R>;
    fn neg(mut self) -> Self {
        for i in self.coeffs.iter_mut() {
            *i *= -R::one()
        }
        self
    }
}

impl<R: Ring> Sub for Unipol<R> {
    type Output = Unipol<R>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, other: Unipol<R>) -> Self {
        self + other.neg()
    }
}

impl<R: Ring> Mul for Unipol<R> {
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
            .fold(Unipol::zero(), Unipol::add)
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

impl<R: Ring> One for Unipol<R> {
    fn one() -> Self {
        Unipol {
            coeffs: vec![R::one()],
        }
    }
}

impl<R: Ring> Semiring for Unipol<R> {
    fn from_nat(n: usize) -> Self {
        if n == 0 {
            Unipol::zero()
        } else {
            Unipol {
                coeffs: vec![R::from_nat(n)],
            }
        }
    }
}

impl<R: Ring> Ring for Unipol<R> {
    fn from_int(n: isize) -> Self {
        if n == 0 {
            Unipol::zero()
        } else {
            Unipol {
                coeffs: vec![R::from_int(n)],
            }
        }
    }
}

pub struct Iter<'a, R: 'a> {
    inner_iter: iter::Map<iter::Enumerate<slice::Iter<'a, R>>, fn((usize, &R)) -> (Power, &R)>,
}

impl<'a, R: 'a> Iterator for Iter<'a, R> {
    type Item = (Power, &'a R);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

impl<'a, R: 'a> iter::FusedIterator for Iter<'a, R> {}
impl<'a, R: 'a> iter::ExactSizeIterator for Iter<'a, R> {}
impl<'a, R: 'a> iter::DoubleEndedIterator for Iter<'a, R> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner_iter.next_back()
    }
}

impl<'a, R: 'a> Unipol<R> {
    pub fn iter(&'a self) -> Iter<'a, R> {
        Iter {
            inner_iter: self.coeffs.iter().enumerate().map(|(a, b)| (Power(a), b)),
        }
    }
}

impl<R: Ring> iter::FromIterator<(Power, R)> for Unipol<R> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Power, R)>,
    {
        let mut coeffs = Vec::new();
        for (Power(i), r) in iter {
            coeffs[i] += r
        }
        Unipol { coeffs }
    }
}

impl<R: Ring> IntoIterator for Unipol<R> {
    type Item = (Power, R);
    type IntoIter = iter::Map<iter::Enumerate<vec::IntoIter<R>>, fn((usize, R)) -> (Power, R)>;
    fn into_iter(self) -> Self::IntoIter {
        self.coeffs
            .into_iter()
            .enumerate()
            .map(|(a, b)| (Power(a), b))
    }
}

impl<R: Ring> Polynomial for Unipol<R> {
    type Monomial = Power;
    type Coeff = R;

    fn lead_term(&self) -> Option<(Power, &R)> {
        self.coeffs
            .iter()
            .enumerate()
            .last()
            .map(|(a, b)| (Power(a), b))
    }

    fn var(_: ()) -> Self {
        Self::x()
    }

    fn terms(&self) -> BTreeMap<Power, &R> {
        self.coeffs
            .iter()
            .enumerate()
            .map(|(a, b)| (Power(a), b))
            .collect()
    }

    fn pop_lead_term(&mut self) -> Option<(Power, R)> {
        let l = self.coeffs.len();
        match self.coeffs.pop() {
            None => None,
            Some(v) => Some((Power(l - 1), v)),
        }
    }
}

impl<R: Field> Div for Unipol<R> {
    type Output = Unipol<R>;
    #[inline]
    fn div(self, other: Self) -> Self {
        self.div_mod(other).0
    }
}

impl<R: Field> Rem for Unipol<R> {
    type Output = Unipol<R>;

    #[inline]
    fn rem(self, other: Self) -> Self {
        self.div_mod(other).1
    }
}

lift_nums_to_ref!(impl for Unipol<R> where R: Ring);
derive_assign_with!(impl DivAssign, div_assign as Div, div for Unipol<K> where K: Field);
derive_assign_with!(impl RemAssign, rem_assign as Rem, rem for Unipol<K> where K: Field);
