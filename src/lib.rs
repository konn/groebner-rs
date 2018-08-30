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
    use std::iter;
    use std::ops::{Add, Mul};
    use Scalar;

    /// Trait corresponding to polynomials.
    /// Minimal implementation: `lead_term`, `split_lead_term`, `terms`, (`var` or `from_terms`) and `lift_map`
    pub trait Polynomial: Ring
    where
        Scalar<Self::Coeff>: Mul<Self, Output = Self>,
    {
        type Monomial: Monomial;
        type Coeff: Ring;
        // type Term: Iterator<Item = (Self::Monomial, &Self::Coeff)>;
        // type TermMut: Iterator<(Self::Monomial, &'a mut Self::Coeff)>

        fn lead_term(&self) -> Option<(Self::Monomial, &Self::Coeff)>;
        fn lead_monom(&self) -> Option<Self::Monomial> {
            self.lead_term().map(|a| a.0)
        }
        fn lead_coeff(&self) -> Option<&Self::Coeff> {
            self.lead_term().map(|a| a.1)
        }

        fn split_lead_term(mut self) -> (Option<(Self::Monomial, Self::Coeff)>, Self) {
            let mopt = self.pop_lead_term();
            (mopt, self)
        }

        fn pop_lead_term(&mut self) -> Option<(Self::Monomial, Self::Coeff)>;

        fn terms(&self) -> BTreeMap<Self::Monomial, &Self::Coeff>;

        fn total_deg(&self) -> usize {
            self.terms()
                .keys()
                .cloned()
                .map(|a| a.total_deg())
                .max()
                .unwrap_or(0)
        }

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

        fn lift_map<T, F>(&self, map: F) -> T
        where
            T: Mul<Self::Coeff, Output = T> + Ring,
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

        fn spol(self, other: Self) -> Self
        where
            Self::Coeff: Field,
        {
            let (mcx, f) = self.split_lead_term();
            let (mx, cx) = mcx.unwrap();
            let (mcy, g) = other.split_lead_term();
            let (my, cy) = mcy.unwrap();
            let m = mx.clone().lcm(my.clone());
            let mx: Self::Monomial = (m / mx).unwrap();
            let my: Self::Monomial = (m / my).unwrap();
            let f = Scalar(cx.clone().recip()) * Self::from_monomial(mx) * f;
            let g = Scalar(cy.clone().recip()) * Self::from_monomial(my) * g;

            f - g
        }

        fn div_mod(self, g: Self) -> (Self, Self)
        where
            Self::Coeff: Field,
        {
            let mut r = self;
            let mut q = Self::zero();
            let (md, g) = g.split_lead_term();
            let (d, c) = md.unwrap();
            while let Some((lt_f, lc_f)) = r.pop_lead_term() {
                match lt_f / d {
                    None => {
                        r += Scalar(lc_f) * Self::from_monomial(lt_f);
                        break;
                    }
                    Some(lt_f) => {
                        let k = lc_f / c.clone();
                        let coe = Scalar(k) * Self::from_monomial(lt_f);
                        q += coe.clone();
                        r -= coe * g.clone();
                    }
                }
            }
            (q, r)
        }

        fn div_mod_polys<I>(mut self, gs: I) -> (Vec<Self>, Self)
        where
            Self::Coeff: Field,
            I: IntoIterator<Item = Self> + Clone,
            <I as IntoIterator>::IntoIter: Clone,
        {
            let mut qs: Vec<Self> = iter::repeat(Self::zero())
                .take(gs.clone().into_iter().count())
                .collect();
            let mut r = Self::zero();
            let gs = gs
                .into_iter()
                .map(|g| {
                    let (mls, g) = g.split_lead_term();
                    (mls.unwrap(), g)
                })
                .enumerate();
            while let Some((lt_f, lc_f)) = self.pop_lead_term() {
                if let Some((i, d, c, g)) = gs
                    .clone()
                    .filter_map(|(i, ((d, c), g))| (lt_f / d).map(|t| (i, t, c, g)))
                    .next()
                {
                    let k = lc_f / c;
                    let coe = Scalar(k) * Self::from_monomial(d);
                    qs[i] += coe.clone();
                    self -= coe * g;
                } else {
                    r += Scalar(lc_f) * Self::from_monomial(lt_f);
                }
            }
            (qs, r)
        }
    }

    pub mod unipol;
    pub use self::unipol::*;

    pub mod ordpol;
    pub use self::ordpol::*;

    pub mod groebner;
    pub use self::groebner::*;

}
