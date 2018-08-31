use entry::*;
use monomial::Monomial;
use polynomial::Polynomial;
use ring::*;
use scalar::*;

use num_traits::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter;
use std::ops::*;

#[inline]
pub fn f5<P>(ideal: Vec<P>) -> Vec<P>
where
    P: Polynomial,
    P::Coeff: Field,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    signature_gb(ideal).into_iter().map(|a| a.1).collect()
}

pub fn signature_gb<P>(ref ideal: Vec<P>) -> Vec<(Vec<P>, P)>
where
    P: Polynomial,
    P::Coeff: Field,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    if ideal.iter().all(Zero::is_zero) {
        return Vec::new();
    }

    // Initialisation
    let mut gs: Vec<(P, Entry<Sig<P>, Vec<P>>)> = Vec::new();
    let n = ideal.len();
    let mut ps: BinaryHeap<_> = (0..n).into_iter().map(|i| to_entry(basis(n, i))).collect();
    let mut syzs: Vec<_> = (0..n)
        .into_iter()
        .flat_map(|i: usize| {
            (0..i).into_iter().map(move |j: usize| {
                to_entry(
                    iter::repeat(P::zero())
                        .take(i)
                        .chain(iter::once(Neg::neg(ideal[j].clone())))
                        .chain(iter::repeat(P::zero()).take(i - j - 1))
                        .chain(iter::once(ideal[i].clone()))
                        .chain(iter::repeat(P::zero()))
                        .take(n)
                        .collect(),
                )
            })
        })
        .collect();

    // Main loop
    while let Some(Entry(g_sig, g)) = ps.pop() {
        let syz_vecs: Vec<_> = syzs.iter().cloned().map(move |a| a.0).collect();
        let go_next = std_crieterion(&g_sig, syz_vecs.as_slice());
        let go_next = go_next || gs.iter().cloned().any(|(_, Entry(s, _))| s == g_sig);
        if go_next {
            continue;
        }
        let (mut h, mut ph) = reduce(ideal.clone(), g, gs.clone());
        if ph.is_zero() {
            syzs.push(to_entry(h));
        } else {
            let c_inv = P::from_coeff(P::Coeff::one() / ph.lead_coeff().unwrap().clone());
            for k in h.iter_mut() {
                *k = c_inv.clone() * k.clone();
            }
            ph *= c_inv;
            for i in gs.iter().cloned().filter_map(|(phg, Entry(_, g))| {
                regular_svector(ph.clone(), h.clone(), phg, g).map(to_entry)
            }) {
                ps.push(i);
            }
            gs.push((ph, to_entry(h)));
        }
    }

    gs.into_iter().map(|(p, Entry(_, a))| (a, p)).collect()
}

#[inline]
fn basis<P>(n: usize, i: usize) -> Vec<P>
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    let mut vec: Vec<P> = iter::repeat(P::zero()).take(n).collect();
    vec[i] = P::one();
    vec
}

#[derive(Debug, Clone)]
struct Sig<P>
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    position: usize,
    coeff: P::Coeff,
    monomial: P::Monomial,
}

impl<P: Polynomial> PartialEq for Sig<P>
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    fn eq(&self, other: &Sig<P>) -> bool {
        self.position == other.position && self.monomial == other.monomial
    }
}

impl<P: Polynomial> Eq for Sig<P>
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
}

impl<P: Polynomial> PartialOrd for Sig<P>
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    fn partial_cmp(&self, other: &Sig<P>) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl<P: Polynomial> Ord for Sig<P>
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    fn cmp(&self, other: &Sig<P>) -> Ordering {
        self.position
            .cmp(&other.position)
            .then(self.monomial.cmp(&other.monomial))
    }
}

fn to_entry<P>(v: Vec<P>) -> Entry<Sig<P>, Vec<P>>
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    Entry(signature(v.clone()), v)
}

fn signature<P, I>(fs: I) -> Sig<P>
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
    I: IntoIterator<Item = P>,
{
    fs.into_iter()
        .enumerate()
        .filter_map(move |(position, f)| {
            f.split_lead_term().0.and_then(|(monomial, coeff)| {
                Some(Sig {
                    position,
                    monomial,
                    coeff,
                })
            })
        })
        .max()
        .unwrap()
}

fn regular_svector<P>(f: P, f_vec: Vec<P>, g: P, g_vec: Vec<P>) -> Option<Vec<P>>
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    f.split_lead_term().0.and_then(move |(lm_f, _)| {
        g.split_lead_term().0.and_then(move |(lm_g, _)| {
            let l = lm_f.clone().lcm(lm_g.clone());
            (l / lm_f).and_then(|lm_f| {
                (l / lm_g).and_then(|lm_g| {
                    let vl: Vec<P> = f_vec
                        .into_iter()
                        .map(|f| f * P::from_monomial(lm_f))
                        .collect();
                    let vr: Vec<P> = g_vec
                        .into_iter()
                        .map(|f| f * P::from_monomial(lm_g))
                        .collect();
                    if signature(vl.clone()) == signature(vr.clone()) {
                        None
                    } else {
                        Some(vl.into_iter().zip(vr).map(|(l, r)| l - r).collect())
                    }
                })
            })
        })
    })
}

fn divs<P>(
    Sig {
        position: i,
        monomial: m,
        ..
    }: &Sig<P>,
    Sig {
        position: j,
        monomial: n,
        ..
    }: &Sig<P>,
) -> bool
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    i == j && m.divides(n)
}

fn std_crieterion<P>(s: &Sig<P>, i: &[Sig<P>]) -> bool
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    i.iter().any(|g| divs(s, g))
}

fn dot<P>(ideal: Vec<P>, g: Vec<P>) -> P
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
{
    ideal
        .into_iter()
        .zip(g)
        .map(|(p, q)| p * q)
        .fold(P::zero(), P::add)
}

fn reduce<P>(ideal: Vec<P>, mut g: Vec<P>, hs: Vec<(P, Entry<Sig<P>, Vec<P>>)>) -> (Vec<P>, P)
where
    P: Polynomial,
    Scalar<P::Coeff>: Mul<P, Output = P>,
    P::Coeff: Field,
{
    let mut phi = dot(ideal.clone(), g.clone());
    let mut r = P::zero();
    while phi != r {
        let (m, c) = (phi.clone() - r.clone())
            .split_lead_term()
            .0
            .unwrap_or((P::Monomial::one(), P::Coeff::zero()));
        match hs
            .iter()
            .cloned()
            .filter_map(|(h, Entry(_, mut hi))| {
                h.lead_term().and_then(|(lm_h, lc_h)| {
                    (m / lm_h).and_then(|lm| {
                        let fac = Scalar(c.clone() / lc_h.clone()) * P::from_monomial(lm);
                        for i in hi.iter_mut() {
                            *i *= fac.clone();
                        }
                        if signature(hi.clone()) < signature(g.clone()) {
                            Some((hi, fac.clone() * h.clone()))
                        } else {
                            None
                        }
                    })
                })
            })
            .next()
        {
            None => r += Scalar(c) * P::from_monomial(m),
            Some((d, ph)) => {
                for (i, p) in Vec::into_iter(d).enumerate() {
                    g[i] -= p;
                }
                phi -= ph;
            }
        }
    }

    (g, phi)
}
