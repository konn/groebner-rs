use groebner_rs::monomial::*;
use groebner_rs::polynomial::groebner::*;
use groebner_rs::polynomial::Ordpol;
use groebner_rs::polynomial::Polynomial;
use groebner_rs::ring::*;
use num_rational::*;
use num_traits::One;

fn main() {
    let x: &Ordpol<Rational, Lex2> = &Ordpol::var(lex2::X);
    let y: &Ordpol<Rational, Lex2> = &Ordpol::var(lex2::Y);
    let f = &(x + y);
    let g = &(x - y);

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

    let x: &Ordpol<Rational, Grevlex2> = &Ordpol::var(grevlex2::X);
    let y: &Ordpol<Rational, Grevlex2> = &Ordpol::var(grevlex2::Y);
    let f = x * x * y - Ordpol::one();
    let g = x * x * x - y * y - x;

    println!(
        "calcGB(x^2 y - 1, x^3 - y^2 - x) = {:?}",
        buchberger(vec![f.clone(), g.clone()])
    );

    println!("f5(x^2 y - 1, x^3 - y^2 - x) = {:?}", f5(vec![f, g]));

    let v: &Ordpol<Rational, Grevlex5> = &Ordpol::var(grevlex5::V);
    let w: &Ordpol<Rational, Grevlex5> = &Ordpol::var(grevlex5::W);
    let x: &Ordpol<Rational, Grevlex5> = &Ordpol::var(grevlex5::X);
    let y: &Ordpol<Rational, Grevlex5> = &Ordpol::var(grevlex5::Y);
    let z: &Ordpol<Rational, Grevlex5> = &Ordpol::var(grevlex5::Z);
    let f = Ordpol::from_nat(35) * y.clone().pow(4)
        - Ordpol::from_nat(30) * x * y.clone().pow(2)
        - Ordpol::from_nat(210) * y.clone().pow(2) * z
        + Ordpol::from_nat(3) * x.clone().pow(2)
        + Ordpol::from_nat(30) * x * z
        - Ordpol::from_nat(105) * z.clone().pow(2)
        + Ordpol::from_nat(140) * y * v
        - Ordpol::from_nat(21) * w;
    let g = Ordpol::from_nat(5) * x * y.clone().pow(3)
        - Ordpol::from_nat(140) * y.clone().pow(3) * z
        - Ordpol::from_nat(3) * x.clone().pow(2) * y
        + Ordpol::from_nat(45) * x * y * z
        - Ordpol::from_nat(420) * y * z.clone().pow(2)
        + Ordpol::from_nat(210) * y.clone().pow(2) * v
        - Ordpol::from_nat(25) * x * v
        + Ordpol::from_nat(70) * z * v
        + Ordpol::from_nat(126) * y * w;
    println!(
        "GB of complex polyns: {:?}",
        buchberger(vec![f.clone(), g.clone()])
    );

    println!("F_5 Gb of complex polyns: {:?}", f5(vec![f, g]));
}
