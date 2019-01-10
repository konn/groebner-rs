use num_traits::*;
use std::ops::*;

#[cfg(test)]
use quickcheck::*;
#[cfg(test)]
use rand::Rng;

/// Monomial multiplicative monoid, endowed with monoidal ordering.
/// A type must satisfy the axioms of ordered free commutative monoids;
/// I.e. a * b = b * a, a * (b * c) = (a * b) * c, 1 <= a, and "a <= b implies a * c <= b * c".
pub trait Monomial: Div<Self, Output = Option<Self>> + Ord + One + Copy {
    type Var: Copy + PartialEq + Eq + Ord;

    /// Returns the list of variables, in decreasing order;
    fn variables() -> Vec<Self::Var>;
    fn var(var: Self::Var) -> Self;
    fn from_exponents(exps: &[(Self::Var, usize)]) -> Self {
        exps.iter()
            .map(|(v, i)| pow(Self::var(*v), *i))
            .fold(Self::one(), Self::mul)
    }
    fn exponent(&self, var: Self::Var) -> usize;

    /// Returns the list of pairs of variable and exponents, in variable decreasing order;
    fn exponents(&self) -> Vec<(Self::Var, usize)> {
        Self::variables()
            .into_iter()
            .map(|v| {
                let c = self.exponent(v);
                (v, c)
            })
            .collect()
    }

    fn total_deg(&self) -> usize {
        self.exponents().iter().map(|(_, n)| n).sum()
    }

    fn divides(&self, other: &Self) -> bool {
        (*other / *self).is_some()
    }

    fn lcm(self, other: Self) -> Self {
        let vec: Vec<_> = self
            .exponents()
            .into_iter()
            .zip(other.exponents().into_iter())
            .map(|((v, m), (_, n))| (v, m.max(n)))
            .collect();
        Self::from_exponents(&vec)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub struct Power(pub usize);

#[cfg(test)]
impl Arbitrary for Power {
    fn arbitrary<G: Gen>(g: &mut G) -> Power {
        Power(g.gen_range(0, ::std::usize::MAX / 3))
    }
}

impl Mul for Power {
    type Output = Power;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self: Power, Power(b): Power) -> Power {
        match self {
            Power(a) => Power(a + b),
        }
    }
}

impl One for Power {
    fn one() -> Power {
        Power(0)
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl Monomial for Power {
    type Var = ();

    fn variables() -> Vec<()> {
        vec![()]
    }

    fn var(_: ()) -> Self {
        Power(1)
    }

    fn exponent(&self, _: ()) -> usize {
        self.0
    }

    fn exponents(&self) -> Vec<((), usize)> {
        vec![((), self.0)]
    }
}

impl Div for Power {
    type Output = Option<Self>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: Power) -> Option<Self> {
        if self.0 < other.0 {
            None
        } else {
            Some(Power(self.0 - other.0))
        }
    }
}

new_monomial!(impl Monomial(X, Y; lex!) for Lex2 in lex2);
new_monomial!(impl Monomial(X, Y; grevlex!) for Grevlex2 in grevlex2);
new_monomial!(impl Monomial(X, Y; grlex!) for Grlex2 in grlex2);

new_monomial!(impl Monomial(X, Y, Z; lex!) for Lex3 in lex3);
new_monomial!(impl Monomial(X, Y, Z; grevlex!) for Grevlex3 in grevlex3);
new_monomial!(impl Monomial(X, Y, Z; grlex!) for Grlex3 in grlex3);

new_monomial!(impl Monomial(W, X, Y, Z; lex!) for Lex4 in lex4);
new_monomial!(impl Monomial(W, X, Y, Z; grevlex!) for Grevlex4 in grevlex4);
new_monomial!(impl Monomial(W, X, Y, Z; grlex!) for Grlex4 in grlex4);

new_monomial!(impl Monomial(V, W, X, Y, Z; lex!) for Lex5 in lex5);
new_monomial!(impl Monomial(V, W, X, Y, Z; grevlex!) for Grevlex5 in grevlex5);
new_monomial!(impl Monomial(V, W, X, Y, Z; grlex!) for Grlex5 in grlex5);

#[cfg(test)]
mod tests;
