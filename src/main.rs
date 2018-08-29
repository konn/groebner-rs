extern crate groebner_rs;
extern crate num_traits;
use groebner_rs::polynomial;
use groebner_rs::polynomial::Unipol;
use groebner_rs::ring::Semiring;
use num_traits::One;

fn main() {
    println!("2 ^ 10 = {}", 2_usize.pow(10));
    use One;
    use Unipol;
    let f: Unipol<isize> = Unipol::x() + One::one();
    let g: Unipol<isize> = Unipol::x() - One::one();
    println!("(x + 1) = {:?}", f);
    println!("(x + 1)(x - 1) = {:?}", f.clone() * g);
    println!("(x + 1) ^ 2 = {:?}", f.clone().pow(2));
    println!("(x + 1)(x + 1) = {:?}", f.clone() * f);
}
