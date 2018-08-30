use num_traits::*;
use std::cmp::Ordering;
use std::ops::*;

/// Monomial multiplicative monoid, endowed with monoidal ordering.
/// A type must satisfy the axioms of ordered free commutative monoids;
/// I.e. a * b = b * a, a * (b * c) = (a * b) * c, 1 <= a, and "a <= b implies a * c <= b * c".
pub trait Monomial: Div<Self, Output = Option<Self>> + Ord + One + Copy {
    type Var: Copy;

    fn variables() -> Vec<Self::Var>;
    fn var(var: &Self::Var) -> Option<Self>;
    fn exponent(&self, var: &Self::Var) -> Option<usize>;
    fn exponents(&self) -> Vec<(Self::Var, usize)> {
        Self::variables()
            .into_iter()
            .map(|v| {
                let c = self.exponent(&v).unwrap();
                (v, c)
            })
            .collect()
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

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Power(pub usize);

impl Mul for Power {
    type Output = Power;
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

    fn var(_: &()) -> Option<Self> {
        Some(Power(1))
    }

    fn exponent(&self, (): &()) -> Option<usize> {
        Some(self.0)
    }

    fn exponents(&self) -> Vec<((), usize)> {
        vec![((), self.0)]
    }
}

impl Div for Power {
    type Output = Option<Self>;

    fn div(self, other: Power) -> Option<Self> {
        if self.0 < other.0 {
            None
        } else {
            Some(Power(self.0 - other.0))
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Lex2(pub [usize; 2]);

impl Mul for Lex2 {
    type Output = Self;
    fn mul(self: Self, Lex2([x2, y2]): Self) -> Self {
        match self {
            Lex2([x1, y1]) => Lex2([x1 + x2, y1 + y2]),
        }
    }
}

impl PartialOrd for Lex2 {
    fn partial_cmp(&self, Lex2([ref x2, ref y2]): &Lex2) -> Option<Ordering> {
        let Lex2([x1, y1]) = self;
        Some(x1.cmp(x2).then(y1.cmp(y2)))
    }
}

impl Ord for Lex2 {
    fn cmp(&self, other: &Lex2) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl One for Lex2 {
    fn one() -> Lex2 {
        Lex2([0, 0])
    }
}

impl Div for Lex2 {
    type Output = Option<Lex2>;

    fn div(self, Lex2([x2, y2]): Lex2) -> Option<Lex2> {
        let Lex2([x1, y1]) = self;
        if x1 >= x2 && y1 >= y2 {
            Some(Lex2([x1 - x2, y1 - y2]))
        } else {
            None
        }
    }
}

impl Monomial for Lex2 {
    type Var = bool;

    fn variables() -> Vec<bool> {
        vec![false, true]
    }

    fn var(b: &bool) -> Option<Lex2> {
        Some(if *b { Lex2([0, 1]) } else { Lex2([1, 0]) })
    }

    fn exponent(&self, p: &bool) -> Option<usize> {
        if *p {
            Some(self.0[1])
        } else {
            Some(self.0[0])
        }
    }

    fn exponents(&self) -> Vec<(bool, usize)> {
        vec![(false, self.0[0]), (true, self.0[1])]
    }
}
