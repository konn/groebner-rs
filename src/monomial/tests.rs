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

macro_rules! check_monom_prop {
    (@build_quick_check
         {$(($var:ident : $typ:ty))*}
         $resl:ident
         $test:ident
         $fn_name: ident
    ) =>
    {
        quickcheck!{
            fn $fn_name($($var: $typ),*) -> $resl {
                super::$test($($var),*)
            }
        }
    };
    (@call_with_args
       ($($rests:tt)*) () $_monom:ident { $($acc:tt)* }
    ) => {
        check_monom_prop!{ @build_quick_check {$($acc)*} $($rests)* }
    };

    (@call_with_args
       ($($rests:tt)*) ($var:ident $($tail:ident)*) $monom:ident { $($acc:tt)* }
    ) => {
        check_monom_prop!{@call_with_args ($($rests)*) ($($tail)*) $monom { $($acc)* ($var: $monom)} }
    };

    ( $fn_name:ident = $test:ident<$monom:ident>($($var:ident)*) -> $resl:ident
    ) =>
    { check_monom_prop!{
        @call_with_args
            ($resl $test $fn_name)
            ($($var)*)
            $monom
            {}
      }
    };
}

macro_rules! check_monom {
    (@proc_list $_monom:ident {}) => {};
    (@proc_list
         $monom:ident
         { ($test:ident($($var:ident)*) -> $resl:ident) $($tail:tt)* }
    ) => {
        check_monom_prop!{
            $test = $test<$monom>($($var)*) -> $resl
        }
        check_monom!{
            @proc_list $monom { $($tail)* }
        }
    };
    ($(
         mod $module:ident = $monom:ident for {
           $(fn $test:ident($($var:ident),*) -> $resl:ident);* $(;)*
         }
      )*
    ) => {
        $(
        mod $module {
            use super::*;
            check_monom!{
                @proc_list $monom { $(($test($($var)*) -> $resl))* }
            }
        })*
    };
}

check_monom!{
    mod lex2 = Lex2 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
    }

    mod lex3 = Lex3 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
    }

    mod lex4 = Lex4 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
    }

    mod lex5 = Lex5 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
    }
}
