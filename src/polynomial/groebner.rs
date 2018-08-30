use monomial::Monomial;
use polynomial::Polynomial;
use ring::*;
use scalar::*;
use std::ops::*;

// Buchberger algorithm with primarity and syzygy criterion.
pub fn buchberger<P: Polynomial>(mut ideal: Vec<P>) -> Vec<P>
where
    Scalar<<P as Polynomial>::Coeff>: Mul<P, Output = P>,
    P::Coeff: Field,
{
    let mut pairs: Vec<_> = (0..ideal.len())
        .flat_map(move |a| (0..a).map(move |b| (a, b)))
        .collect();
    println!("pairs: {:?}", pairs);
    let mut n = ideal.len();
    while let Some((i, j)) = pairs.pop() {
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
                .all(|(k, o)| (*k, *o) != (k1, l1) && (*k, *o) != (k2, l2));
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
            for i in 0..n {
                pairs.push((n, i));
            }
            n += 1;
        }
    }
    ideal
}
