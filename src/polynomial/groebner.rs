use polynomial::Polynomial;
use ring::*;
use scalar::*;
use std::ops::*;

pub fn buchberger<'a, P: Polynomial<'a>>(mut ideal: Vec<P>) -> Vec<P>
where
    Scalar<<P as Polynomial<'a>>::Coeff> : Mul<P, Output=P>,
    P::Coeff: Field,
{
    let mut pairs : Vec<_> = (0..ideal.len()).flat_map(move |a| (0..a).map(move |b| (a,b))).collect();
    println!("pairs: {:?}", pairs);
    let mut n = ideal.len();
    while let Some((i, j)) = pairs.pop() {
        let (f, g) = (ideal[i].clone(), ideal[j].clone());
        let (_, s) = f.spol(g).div_mod_polys(ideal.clone());
        if !s.is_zero() {
            ideal.push(s);
            for i in 0..n {
                pairs.push((n, i));
            }
            n += 1;
        }
    };
    ideal
}
