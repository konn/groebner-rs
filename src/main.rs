extern crate groebner_rs;
extern crate num_rational;
extern crate num_traits;
use groebner_rs::monomial::*;
use groebner_rs::polynomial::Ordpol;
use groebner_rs::polynomial::Polynomial;
use groebner_rs::polynomial::Unipol;
use groebner_rs::ring::Semiring;
use num_rational::*;
use num_traits::One;

fn main() {
    use Unipol;
    let x: &Unipol<isize> = &Unipol::x();
    let f = &(x + &One::one());
    let g = &(x - &One::one());
    let one = Unipol::<isize>::one();
    println!("x = {:?}", x);
    println!("(x + 1) = {:?}", f);
    println!("(x - 1) = {:?}", x.clone() - one.clone());
    println!("(1 - x) = {:?}", &one - x);
    println!("(x + 1)(x - 1) = {:?}", f * g);
    println!("(x + 1) ^ 2 = {:?}", f.clone().pow(2));
    println!("(x + 1)(x + 1) = {:?}", f * f);
    println!("(x + 1) - (x + 1) = {:?}", f - f);

    let x: &Ordpol<Rational, Lex2> = &Ordpol::var(false).unwrap();
    let y: &Ordpol<Rational, Lex2> = &Ordpol::var(true).unwrap();
    let f = &(x + y);
    let g = &(x - y);
    println!("(x + y) = {:?}", f);
    println!("(x - y) = {:?}", f);
    println!("(x + y)(x - y) = {:?}", f * g);

    println!(
        "(x+y)(x-y) / (x- y + 1) = {:?}",
        f * g / (g + Ordpol::one())
    );
    println!(
        "(x+y)(x-y) % (x- y + 1) = {:?}",
        f * g % (g + Ordpol::one())
    );
}
