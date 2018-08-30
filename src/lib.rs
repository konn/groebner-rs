extern crate num_integer;
extern crate num_rational;
extern crate num_traits;

#[macro_use]
mod macros;

pub mod monomial;
pub mod ring;
pub mod scalar;

pub use monomial::*;
pub use ring::*;
pub use scalar::*;

pub mod polynomial {
    use monomial::*;
    use num_traits::*;
    use ring::*;
    use std::collections::BTreeMap;
    use std::ops::{Add, Mul};
    use Scalar;

    /// Trait corresponding to polynomials.
    /// Minimal implementation: `lead_term`, `split_lead_term`, `terms`, (`var` or `from_terms`) and `lift_map`
    pub trait Polynomial<'a>: Ring
    where
        Scalar<Self::Coeff>: Mul<Self, Output = Self>,
    {
        type Monomial: Monomial;
        type Coeff: Ring;
        // type Term: Iterator<Item = (Self::Monomial, &Self::Coeff)>;
        // type TermMut: Iterator<(Self::Monomial, &'a mut Self::Coeff)>

        fn lead_term(&'a self) -> Option<(Self::Monomial, &'a Self::Coeff)>;
        fn lead_monom(&'a self) -> Option<Self::Monomial> {
            self.lead_term().map(|a| a.0)
        }
        fn lead_coeff(&'a self) -> Option<&'a Self::Coeff> {
            self.lead_term().map(|a| a.1)
        }

        fn split_lead_term(mut self) -> (Option<(Self::Monomial, Self::Coeff)>, Self) {
            let mopt = self.pop_lead_term();
            (mopt, self)
        }

        fn pop_lead_term(&mut self) -> Option<(Self::Monomial, Self::Coeff)>;

        fn terms(&'a self) -> BTreeMap<Self::Monomial, &'a Self::Coeff>;

        fn var(v: <Self::Monomial as Monomial>::Var) -> Option<Self> {
            match <Self::Monomial as Monomial>::var(&v) {
                None => None,
                Some(p) => Some(Self::from_terms(
                    [(p, One::one())].iter().cloned().collect(),
                )),
            }
        }

        fn from_monomial(monomial: Self::Monomial) -> Self {
            monomial
                .exponents()
                .into_iter()
                .map(|(v, i)| pow(Self::var(v).unwrap(), i))
                .fold(Self::one(), Self::mul)
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

    pub mod unipol;
    pub use self::unipol::*;

    pub mod ordpol;
    pub use self::ordpol::*;
}
