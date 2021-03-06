use crate::entry::*;
use crate::monomial::Monomial;
use crate::polynomial::Polynomial;
use crate::ring::*;
use crate::scalar::*;
use num_traits::One;
use std::cmp;
use std::collections::BinaryHeap;
use std::ops::*;

/// The sugar selection strategy for critical pairs
#[inline]
pub fn sugar<P>(f: &P, g: &P) -> cmp::Reverse<usize>
where
    P: Polynomial,
    Scalar<<P as Polynomial>::Coeff>: Mul<P, Output = P>,
{
    let lm_f = f.lead_monom().unwrap_or_else(One::one);
    let lm_g = g.lead_monom().unwrap_or_else(One::one);
    let lcm_fg = lm_f.lcm(lm_g);
    let total_f = f.total_deg() - lm_f.total_deg();
    let total_g = g.total_deg() - lm_g.total_deg();
    cmp::Reverse(total_f.max(total_g) + lcm_fg.total_deg())
}

/// Buchberger algorithm with sugar strategy, coprimarity and syzygy criterion.
#[inline]
pub fn buchberger<P: Polynomial>(ideal: Vec<P>) -> Vec<P>
where
    Scalar<<P as Polynomial>::Coeff>: Mul<P, Output = P>,
    P::Coeff: Field,
{
    buchberger_with(sugar, ideal)
}

/// Buchberger algorithm with coprimarity and syzygy criterion,
/// which accepts selection strategy as a weighting function.
/// This function processes critical pairs in heavier-first manner.
pub fn buchberger_with<W: Ord, F, P: Polynomial>(calc_weight: F, mut ideal: Vec<P>) -> Vec<P>
where
    F: Fn(&P, &P) -> W + Copy,
    Scalar<<P as Polynomial>::Coeff>: Mul<P, Output = P>,
    P::Coeff: Field,
{
    let mut pairs = BinaryHeap::new();
    for i in 0..ideal.len() {
        for j in 0..i {
            // Registering ciritcal pairs, with a weight for selection strategy
            pairs.push(Entry(calc_weight(&ideal[i], &ideal[j]), (i, j)))
        }
    }
    let mut n = ideal.len();
    while let Some(Entry(_, (i, j))) = pairs.pop() {
        let (lt_f, lt_g) = (
            &ideal[0].lead_monom().unwrap(),
            &ideal[1].lead_monom().unwrap(),
        );
        let lcm_fg = lt_f.lcm(*lt_g);

        // Primarity check
        if lcm_fg == *lt_f * *lt_g {
            continue;
        }

        // Syzygy test
        let syz = ideal.iter().enumerate().any(|(l, h)| {
            let [k1, l1] = [i.min(i), i.max(l)];
            let [k2, l2] = [j.min(j), j.max(l)];
            let distinct = pairs
                .iter()
                .all(|Entry(_, (k, o))| (*k, *o) != (k1, l1) && (*k, *o) != (k2, l2));
            l != i && l != j && distinct && h.lead_monom().unwrap().divides(&lcm_fg)
        });
        if syz {
            continue;
        }

        // S-test
        let (f, g) = (ideal[i].clone(), ideal[j].clone());
        let (_, s) = f.spol(g).div_mod_polys(ideal.clone());
        if !s.is_zero() {
            ideal.push(s);
            for k in 0..n {
                pairs.push(Entry(calc_weight(&ideal[k], &ideal[n]), (n, k)));
            }
            n += 1;
        }
    }
    ideal
}

pub mod signature;
pub use self::signature::f5;
