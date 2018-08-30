use monomial::*;
use num_traits::*;
use polynomial::Polynomial;
use ring::*;
use scalar::*;
use std::collections::btree_map as btm;
use std::collections::BTreeMap;
use std::ops::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Ordpol<R, X>(BTreeMap<X, R>)
where
    X: Monomial;

impl<R: Zero + AddAssign<R> + Clone, X: Monomial> Add for Ordpol<R, X> {
    type Output = Ordpol<R, X>;
    fn add(mut self, other: Ordpol<R, X>) -> Ordpol<R, X> {
        for (k, v) in other.0 {
            let to_remove = match self.0.entry(k) {
                btm::Entry::Vacant(e) => {
                    e.insert(v);
                    false
                }
                btm::Entry::Occupied(mut e) => {
                    let u = e.get_mut();
                    *u += v;
                    u.is_zero()
                }
            };
            if to_remove {
                self.0.remove(&k);
            }
        }
        self
    }
}

impl<R: Zero + AddAssign<R> + Clone, X: Monomial> Zero for Ordpol<R, X> {
    fn zero() -> Self {
        Ordpol(BTreeMap::new())
    }

    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }
}

impl<R, X> Mul<Ordpol<R, X>> for Scalar<R>
where
    R: One + Zero + Clone,
    X: Monomial,
{
    type Output = Ordpol<R, X>;
    fn mul(self, Ordpol(dic): Ordpol<R, X>) -> Ordpol<R, X> {
        if self.0.is_zero() {
            Ordpol(BTreeMap::new())
        } else {
            Ordpol(
                dic.into_iter()
                    .filter_map(|(a, r)| {
                        let v = self.0.clone() * r;
                        if v.is_zero() {
                            None
                        } else {
                            Some((a, v))
                        }
                    })
                    .collect(),
            )
        }
    }
}

impl<R: One + AddAssign<R> + Zero + Clone, X: Monomial> Mul for Ordpol<R, X> {
    type Output = Ordpol<R, X>;

    fn mul(self, other: Ordpol<R, X>) -> Ordpol<R, X> {
        self.0
            .into_iter()
            .flat_map(|(m, c)| {
                other.0.clone().into_iter().map(move |(n, d)| {
                    let mut dic: BTreeMap<X, R> = BTreeMap::new();
                    let v = c.clone() * d;
                    if !v.is_zero() {
                        dic.entry(m.clone() * n).or_insert(v);
                    }
                    Ordpol(dic)
                })
            })
            .fold(Ordpol::zero(), Ordpol::<R, X>::add)
    }
}

impl<R: One + AddAssign<R> + Zero + Clone, X: Monomial> One for Ordpol<R, X> {
    fn one() -> Ordpol<R, X> {
        Ordpol([(X::one(), R::one())].into_iter().cloned().collect())
    }
}

impl<R: Neg<Output = R> + One + Zero + Clone, X: Monomial> Neg for Ordpol<R, X> {
    type Output = Ordpol<R, X>;
    fn neg(self) -> Self {
        Ordpol(self.0.into_iter().map(|(a, b)| (a, b.neg())).collect())
    }
}

impl<R: Ring, X: Monomial> Sub for Ordpol<R, X> {
    type Output = Ordpol<R, X>;
    fn sub(self, other: Ordpol<R, X>) -> Self {
        self + other.neg()
    }
}

impl<R: Ring, X: Monomial> Semiring for Ordpol<R, X> {
    fn from_nat(i: usize) -> Self {
        if i == 0 {
            Ordpol(BTreeMap::new())
        } else {
            Ordpol([(X::one(), R::from_nat(i))].iter().cloned().collect())
        }
    }
}
impl<R: Ring, X: Monomial> Ring for Ordpol<R, X> {
    fn from_int(i: isize) -> Self {
        if i == 0 {
            Ordpol(BTreeMap::new())
        } else {
            Ordpol([(X::one(), R::from_int(i))].iter().cloned().collect())
        }
    }
}

impl<R: Ring, X: Monomial> Polynomial for Ordpol<R, X> {
    type Monomial = X;
    type Coeff = R;

    fn var(v: X::Var) -> Self {
        Ordpol([(X::var(v), R::one())].iter().cloned().collect())
    }

    fn lead_term(&self) -> Option<(X, &R)> {
        self.0.iter().next_back().map(|(k, v)| (k.clone(), v))
    }

    fn terms(&self) -> BTreeMap<X, &R> {
        self.0.iter().map(move |(k, v)| (k.clone(), v)).collect()
    }

    fn pop_lead_term(&mut self) -> Option<(X, R)> {
        match self.0.keys().cloned().next_back() {
            None => None,
            Some(k) => self.0.remove(&k).map(|a| (k, a)),
        }
    }
}

lift_nums_to_ref!(impl for Ordpol<R, X> where R: Ring, X: Monomial);

impl<K: Field, X: Monomial> Div for Ordpol<K, X> {
    type Output = Ordpol<K, X>;
    #[inline]
    fn div(self, other: Self) -> Self {
        self.div_mod(other).0
    }
}

impl<K: Field, X: Monomial> Rem for Ordpol<K, X> {
    type Output = Ordpol<K, X>;

    #[inline]
    fn rem(self, other: Self) -> Self {
        self.div_mod(other).1
    }
}

derive_assign_with!(impl DivAssign, div_assign as Div, div for Ordpol<K, X> where K: Field, X: Monomial);
derive_assign_with!(impl RemAssign, rem_assign as Rem, rem for Ordpol<K, X> where K: Field, X: Monomial);
lift_binop_to_ref!(impl Div, div for Ordpol<K, X> where K: Field, X: Monomial);
lift_binop_to_ref!(impl Rem, rem for Ordpol<K, X> where K: Field, X: Monomial);
