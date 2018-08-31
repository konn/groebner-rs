macro_rules! lex {
    ($e:expr) => {
          |xs: [usize; $e], ys: [usize; $e]| -> ::std::cmp::Ordering {
              for (i, j) in xs.into_iter().zip(ys.into_iter()) {
                  match i.cmp(j) {
                      ::std::cmp::Ordering::Equal => {
                          continue;
                      }
                      ord => {
                          return ord;
                      }
                  }
              }
              return ::std::cmp::Ordering::Equal;
          }
    };
}

macro_rules! revlex {
    ($e:expr) => {
         |xs: [usize; $e], ys: [usize; $e]| -> ::std::cmp::Ordering {
              for (i, j) in ys.into_iter().rev().zip(xs.into_iter().rev()) {
                  match i.cmp(j) {
                      ::std::cmp::Ordering::Equal => {
                          continue;
                      }
                      ord => {
                          return ord;
                      }
                  }
              }
              return ::std::cmp::Ordering::Equal;
          }
    };
}

macro_rules! grlex {
    ($e:expr) => {
         |xs: [usize; $e], ys: [usize; $e]| -> ::std::cmp::Ordering {
              let wx : usize = xs.into_iter().sum();
              let wy : usize = ys.into_iter().sum();
              wx.cmp(&wy).then(lex!($e)(xs, ys))
         }
        
    };
}

macro_rules! grevlex {
    ($e:expr) => {
          |xs: [usize; $e], ys: [usize; $e]| -> ::std::cmp::Ordering {
              let wx : usize = xs.into_iter().sum();
              let wy : usize = ys.into_iter().sum();
              wx.cmp(&wy).then(revlex!($e)(xs, ys))
          }
    };
}

macro_rules! new_monomial {
    (impl Monomial($arity:expr; $($var:ident)+; $cmp:expr)for $monom:ident in $mod:ident) => {
        pub mod $mod {
            use ::num_traits::*;
            use ::std::cmp as _cmp;
            use ::std::ops::*;
            use $crate::monomial;

            #[cfg(test)]
            use ::quickcheck::Arbitrary;
            #[cfg(test)]
            use ::quickcheck::Gen;
            #[cfg(test)]
            use ::rand::Rng;

            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub enum Var {
                $($var,)*
            }

            #[cfg(test)]
            impl Arbitrary for Var {
                fn arbitrary<G: Gen>(g: &mut G) -> Var {
                    *g.choose(&[$($var,)*]).unwrap()
                }
            }

            fn to_idx(v: Var) -> usize {
                let mut n = 0;
                $(
                if let Var::$var = v {
                    return n
                }
                n += 1;
                )*;
                return n;
            }

            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub struct $monom(pub [usize; $arity]);

            #[cfg(test)]
            impl Arbitrary for $monom {
                fn arbitrary<G: Gen>(g: &mut G) -> $monom {
                    let mut arr = [0; $arity];
                    let mut vec: Vec<_> = ::std::iter::repeat(0).take($arity).collect();
                    for i in 0..$arity {
                        vec[i] = g.gen_range(0, ::std::usize::MAX / ((1 + $arity) * 2));
                    }
                    arr.copy_from_slice(&vec[..$arity]);
                    $monom(arr)
                }
            }

            impl PartialOrd for Var {
                fn partial_cmp(&self, other: &Self) -> Option<_cmp::Ordering> {
                    Some(Var::cmp(self, other))
                }
            }

            impl Ord for Var {
                fn cmp(&self, other: &Self) -> _cmp::Ordering {
                    to_idx(*other).cmp(&to_idx(*self))
                }
            }

            impl Mul for $monom {
                type Output = $monom;
                fn mul(self, other: $monom) -> $monom {
                    let mut arr = [0; $arity];
                    let vec: Vec<_> = self
                        .0
                        .into_iter()
                        .zip(other.0.into_iter())
                        .map(|(a, b)| a + b)
                        .collect();
                    arr.copy_from_slice(&vec[..$arity]);
                    $monom(arr)
                }
            }

            impl<'a, 'b> Mul<&'b $monom> for &'a $monom {
                type Output = $monom;
                fn mul(self, other: &'b $monom) -> $monom {
                    Mul::mul(*self, *other)
                }
            }

            impl<'a> Mul<$monom> for &'a $monom {
                type Output = $monom;
                fn mul(self, other: $monom) -> $monom {
                    Mul::mul(*self, other)
                }
            }

            impl<'a> Mul<&'a $monom> for $monom {
                type Output = $monom;
                fn mul(self, other: &'a $monom) -> $monom {
                    Mul::mul(self, *other)
                }
            }

            impl One for $monom {
                fn one() -> $monom {
                    $monom([0; $arity])
                }
            }

            impl Div for $monom {
                type Output = Option<$monom>;

                fn div(self, other: $monom) -> Option<$monom> {
                    if self.0.iter().zip(other.0.iter()).all(|(i, j)| i >= j) {
                        let mut arr = [0 ; $arity];
                        let vec: Vec<_> =
                            self.0.into_iter()
                                .zip(other.0.into_iter())
                                .map(|(i, j)| i - j).collect();
                        arr.copy_from_slice(&vec[..$arity]);
                        Some($monom(arr))
                    } else {
                        None
                    }
                }
            }

            impl PartialOrd for $monom {
                fn partial_cmp(&self, other: &$monom) -> Option<_cmp::Ordering> {
                    Some($cmp(self.0, other.0))
                }
            }

            impl Ord for $monom {
                fn cmp(&self, other: &$monom) -> _cmp::Ordering {
                    $cmp(self.0, other.0)
                }
            }

            impl monomial::Monomial for $monom {
                type Var = Var;

                fn variables() -> Vec<Var> {
                    vec![$(Var::$var,)*]
                }

                fn var(v: Var) -> Self {
                    let mut arr = [0 ; $arity];
                    arr[to_idx(v)] = 1;
                    $monom(arr)
                }

                fn exponent(&self, v: Var) -> usize {
                    self.0[to_idx(v)]
                }

                fn exponents(&self) -> Vec<(Var, usize)> {
                    vec![$(Var::$var,)*].into_iter().zip(self.0.iter().cloned()).collect()
                }
            }
            pub use self::Var::{$($var,)*};

        }
        pub use $mod::$monom;
    };
}
