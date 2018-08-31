#![allow(non_snake_case)]
use super::*;
use quickcheck::*;
use std::cmp::Ordering::*;

fn mul_resp_ord<X: Monomial>(a: X, b: X, c: X) -> bool {
    match a.cmp(&b) {
        Equal => a * c == b * c,
        Less => a * c < b * c,
        Greater => a * c > b * c,
    }
}

fn positive<X: Monomial>(a: X) -> bool {
    a == X::one() || a > X::one()
}

macro_rules! quickcheck_monom {
    (@build_quick_check
         {$(($var:ident : $typ:ty))*}
         $resl:ident
         $test:ident
         $fn_name: ident
    ) =>
    {
        quickcheck!{
            fn $fn_name($($var: $typ),*) -> $resl {
                $test($($var),*)
            }
        }
    };
    (@call_with_args
       ($($rests:tt)*) () $_monom:ident { $($acc:tt)* }
    ) => {
        quickcheck_monom!{ @build_quick_check {$($acc)*} $($rests)* }
    };

    (@call_with_args
       ($($rests:tt)*) ($var:ident $($tail:ident)*) $monom:ident { $($acc:tt)* }
    ) => {
        quickcheck_monom!{@call_with_args ($($rests)*) ($($tail)*) $monom { $($acc)* ($var: $monom)} }
    };

    (@build_single
         $resl:ident
         $test:ident($($var:ident)*)
         for ($fn_name: ident, $monom:ident)
    ) =>
    { quickcheck_monom!{
        @call_with_args
            ($resl $test $fn_name)
            ($($var)*)
            $monom
            {}
      }
    };
    (@proc_list $resl:ident $test:ident($($var:ident)*) {}) => {};
    (@proc_list
         $resl:ident
         $test:ident($($var:ident)*)
         { $monom:ident $($tail:tt)* }
    ) => {
        quickcheck_monom!{
            @build_single $resl $test($($var)*) for ($monom, $monom)
        }
        quickcheck_monom!{
            @proc_list $resl $test($($var)*) { $($tail)*  }
        }
    };
    ($($test:ident($($var:ident),*) -> $resl:ident for { $($prs:tt),* $(,)* })* ) => {
        $(
        mod $test {
            use super::*;
            quickcheck_monom!{
                @proc_list $resl $test($($var)*) { $($prs)* }
            }
        }
        )*
    };
}

quickcheck_monom!{
    mul_resp_ord(xs, ys, zs) -> bool for {
        Lex2, Lex3, Lex4, Lex5,
        Grlex2, Grlex3, Grlex4, Grlex5,
        Grevlex2, Grevlex3, Grevlex4, Grevlex5,
    }

    positive(xs) -> bool for {
        Lex2, Lex3, Lex4, Lex5,
        Grlex2, Grlex3, Grlex4, Grlex5,
        Grevlex2, Grevlex3, Grevlex4, Grevlex5,
    }
}
