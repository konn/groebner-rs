extern crate groebner_rs;
extern crate num_traits;
use groebner_rs::monomial::*;
use groebner_rs::polynomial::Ordpol;
use groebner_rs::polynomial::Polynomial;
use groebner_rs::polynomial::Unipol;
use groebner_rs::ring::Semiring;
use num_traits::One;

fn main() {
    use Unipol;
    let x: &Unipol<isize> = &Unipol::x();
    let f = &(x + &One::one());
    let g = &(x - &One::one());
    println!("(x + 1) = {:?}", f);
    println!("(x + 1)(x - 1) = {:?}", f * g);
    println!("(x + 1) ^ 2 = {:?}", f.clone().pow(2));
    println!("(x + 1)(x + 1) = {:?}", f * f);

    let x: &Ordpol<isize, Lex2> = &Ordpol::var(false).unwrap();
    let y: &Ordpol<isize, Lex2> = &Ordpol::var(true).unwrap();
    let f = x + y;
    let g = x - y;
    println!("(x + y) = {:?}", f);
    println!("(x - y) = {:?}", f);
    println!("(x + y)(x - y) = {:?}", f * g);
}
