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
    (@impl_monom ($($var:ident)*) ($($vecs:expr);*) ($cmp:expr) $monom:ident $mod:ident) => {
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
                    *g.choose(&[$($var),*]).unwrap()
                }
            }

            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub struct $monom(pub [usize; $monom::VAR_COUNT]);

            impl $monom {
                pub const VAR_COUNT : usize = new_monomial!(@count_vars $($var)*);
                fn calc_index(v: Var) -> usize {
                    let mut n = 0;
                    $(
                    if let Var::$var = v {
                        return n
                    }
                    n += 1;
                    )*;
                    return n;
                }
                $(
                pub const $var: $monom = $monom($vecs);
                )*
            }

            #[cfg(test)]
            impl Arbitrary for $monom {
                fn arbitrary<G: Gen>(g: &mut G) -> $monom {
                    let mut arr = [0; $monom::VAR_COUNT];
                    let mut vec: Vec<_> = ::std::iter::repeat(0).take($monom::VAR_COUNT).collect();
                    for i in 0..$monom::VAR_COUNT {
                        vec[i] = g.gen_range(0, ::std::usize::MAX / ((1 + $monom::VAR_COUNT) * 2));
                    }
                    arr.copy_from_slice(&vec[..$monom::VAR_COUNT]);
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
                    $monom::calc_index(*other).cmp(&$monom::calc_index(*self))
                }
            }

            impl Mul for $monom {
                type Output = $monom;
                #[allow(clippy::suspicious_arithmetic_impl)]
                fn mul(self, other: $monom) -> $monom {
                    let mut arr = [0; $monom::VAR_COUNT];
                    let vec: Vec<_> = self
                        .0
                        .into_iter()
                        .zip(other.0.into_iter())
                        .map(|(a, b)| a + b)
                        .collect();
                    arr.copy_from_slice(&vec[..$monom::VAR_COUNT]);
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
                    $monom([0; $monom::VAR_COUNT])
                }
            }

            impl Div for $monom {
                type Output = Option<$monom>;
                #[allow(clippy::suspicious_arithmetic_impl)]
                fn div(self, other: $monom) -> Option<$monom> {
                    if self.0.iter().zip(other.0.iter()).all(|(i, j)| i >= j) {
                        let mut arr = [0 ; $monom::VAR_COUNT];
                        let vec: Vec<_> =
                            self.0.into_iter()
                                .zip(other.0.into_iter())
                                .map(|(i, j)| i - j).collect();
                        arr.copy_from_slice(&vec[..$monom::VAR_COUNT]);
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
                    match v {
                        $($var => $monom::$var,)*
                    }
                }

                fn exponent(&self, v: Var) -> usize {
                    self.0[$monom::calc_index(v)]
                }

                fn exponents(&self) -> Vec<(Var, usize)> {
                    vec![$(Var::$var,)*].into_iter().zip(self.0.iter().cloned()).collect()
                }
            }
            pub use self::Var::{$($var,)*};

        }
        pub use self::$mod::$monom;
    };
    (@build_vars_rec
        ()
        ($($v:ident)*)
        [$($_d:expr),*]
        ($([$($entry:expr),* ]);* $(;)*)
        ($cmp:expr) $monom:ident $mod:ident
    ) => {
        new_monomial!{
            @impl_monom ($($v)*) ($([$($entry),*]);*) ($cmp) $monom $mod
        }
    };
    (@build_vars_rec
       ($v:ident $($tail:ident)*)          // Vars to be processed
       ($($vars:ident)*)                   // Processed vars
       [$($d:expr),*]                        // Current prefix
       ($([$($entry:expr),*]);*)            // Accumulated results
       ($cmp:expr) $monom:ident $mod:ident // Continuation
    ) => {
        new_monomial!{
          @build_vars_rec
             ($($tail)*)
             ($($vars)* $v)
             [$($d,)* 0]
             ($([$($entry,)* 0];)* [$($d,)* 1]) ($cmp) $monom $mod
        }
    };
    (@impl_with_vars ($($v:ident)*) ($cmp:expr) $monom:ident $mod:ident) => {
        new_monomial!{ @build_vars_rec ($($v)*) () [] () ($cmp) $monom $mod }
    };
    (@count_vars) => { 0usize };
    (@count_vars $v:ident $($rest:tt)*) => {1usize + new_monomial!(@count_vars $($rest)*) };
    (impl Monomial($($var:ident),*; $cmp:ident!) for $monom:ident in $mod:ident) => {
        new_monomial!{impl Monomial($($var),*; $cmp!($monom::VAR_COUNT))for $monom in $mod}
    };
    (impl Monomial($($var:ident),*; $cmp:expr) for $monom:ident in $mod:ident) => {
        new_monomial!{ @impl_with_vars ($($var)*) ($cmp) $monom $mod }
    };
}
