extern crate num_traits;
pub mod monomial;

pub mod ring;

use num_traits::*;
use ring::*;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub struct Scalar<T>(T);

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

pub mod polynomial {
    use monomial::Power;
    use monomial::*;
    use num_traits::*;
    use ring::*;
    // use std::collections::btree_map as btm;
    use std::collections::BTreeMap;
    use std::iter;
    use std::ops::{Add, Mul, Neg, Sub};
    use Scalar;

    pub trait Polynomial<'a>: Ring
    where
        Scalar<Self::Coeff>: Mul<Self, Output = Self>,
    {
        type Monomial: Monomial;
        type Coeff: Ring;

        fn lead_term(&'a self) -> Option<(Self::Monomial, &'a Self::Coeff)>;
        fn lead_monom(&'a self) -> Option<Self::Monomial> {
            self.lead_term().map(|a| a.0)
        }
        fn lead_coeff(&'a self) -> Option<&'a Self::Coeff> {
            self.lead_term().map(|a| a.1)
        }

        // fn terms_mut(&mut self) -> &mut BTreeMap<Self::Monomial, Self::Coeff>;
        fn terms(&'a self) -> BTreeMap<Self::Monomial, &'a Self::Coeff>;

        // fn iter(&'a self) -> btm::Iter<&'a Self::Monomial, &'a Self::Coeff> {
        //     self.terms().iter()
        // }
        // fn iter_mut(&mut self) -> btm::IterMut<Self::Monomial, Self::Coeff> {
        //     self.terms_mut().iter_mut()
        // }

        fn var(v: <Self::Monomial as Monomial>::Var) -> Option<Self> {
            match <Self::Monomial as Monomial>::var(&v) {
                None => None,
                Some(p) => Some(Self::from_terms(
                    [(p, Self::Coeff::one())].iter().cloned().collect(),
                )),
            }
        }
        fn from_terms(terms: BTreeMap<Self::Monomial, Self::Coeff>) -> Self {
            terms
                .into_iter()
                .map(|(m, c)| {
                    Scalar(c)
                        * m.exponents()
                            .into_iter()
                            .map(|(v, n)| Self::var(v).unwrap().pow(n))
                            .fold(Self::one(), Mul::mul)
                })
                .fold(Self::zero(), Add::add)
        }

        fn lift_map<T, F>(&'a self, map: F) -> T
        where
            T: Mul<Self::Coeff, Output = T> + Ring + 'a,
            F: Fn(&<Self::Monomial as Monomial>::Var) -> T,
        {
            self.terms()
                .iter()
                .map(|(m, c)| {
                    Self::Monomial::variables()
                        .iter()
                        .map(|v| map(v).pow(m.exponent(v).unwrap_or(0)))
                        .fold(T::one(), |a, b| a * b) * (*c).clone()
                })
                .fold(T::zero(), |a, b| a + b)
        }
    }

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
}
