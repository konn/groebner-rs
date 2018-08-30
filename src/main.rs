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

    let mut ideal = vec![x * y - Ordpol::one(), y * y - Ordpol::one()];
    let p = x * x * y + x * y * y + y * y;
    println!(
        "(x^2 y + x y^2 + y^2) /% (x y - 1, y^2 - 1) = {:?}",
        p.clone().div_mod_polys(ideal.clone())
    );
    ideal.reverse();
    println!(
        "(x^2 y + x y^2 + y^2) /% (y^2 - 1, x y - 1) = {:?}",
        p.div_mod_polys(ideal)
    );
}
