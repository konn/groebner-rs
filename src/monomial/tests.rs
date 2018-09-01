use super::*;
use quickcheck::*;
use std::cmp::Ordering::*;
use std::iter;

fn mul_resp_ord<X: Monomial>(a: X, b: X, c: X) -> bool {
    match a.cmp(&b) {
        Equal => a * c == b * c,
        Less => a * c < b * c,
        Greater => a * c > b * c,
    }
}

fn one_is_unit<X: Monomial>(xs: X) -> bool {
    xs * X::one() == xs && xs == X::one() * xs
}

fn mul_assoc<X: Monomial>(xs: X, ys: X, zs: X) -> bool {
    xs * (ys * zs) == (xs * ys) * zs
}

fn positive<X: Monomial>(a: X) -> bool {
    a == X::one() || a > X::one()
}

fn lcm_divisible<X: Monomial>(xs: X, ys: X) -> bool {
    let lcm = xs.lcm(ys);
    xs.divides(&lcm) && ys.divides(&lcm)
}

fn lcm_least<X: Monomial>(xs: X, ys: X) -> bool {
    let lcm = xs.lcm(ys).exponents();
    (0..lcm.len()).all(|i| {
        let mut decreased = lcm.clone();
        decreased[i].1 -= 1;
        let decreased = X::from_exponents(&decreased[0..]);
        !(xs.divides(&decreased) && ys.divides(&decreased))
    })
}

fn var_decreasing<X: Monomial>() -> bool {
    let vars = X::variables()
        .into_iter()
        .map(X::var)
        .chain(iter::once(X::one()));
    vars.clone().zip(vars.skip(1)).all(|(x, y)| x > y)
}

fn resp_total_deg<X: Monomial>(xs: X, ys: X) -> TestResult {
    let deg_x = xs.total_deg();
    let deg_y = ys.total_deg();
    match deg_x.cmp(&deg_y) {
        Equal => TestResult::discard(),
        Less => TestResult::from_bool(xs < ys),
        Greater => TestResult::from_bool(xs > ys),
    }
}

macro_rules! check_monom_prop {
    (@build_quick_check
         $monom:ident
         {$(($var:ident : $typ:ty))*}
         $resl:ident
         $test:ident
         $fn_name: ident
    ) =>
    {
        quickcheck!{
            fn $fn_name($($var: $typ),*) -> $resl {
                super::$test::<$monom>($($var),*)
            }
        }
    };
    (@call_with_args
       ($($rests:tt)*) () $monom:ident { $($acc:tt)* }
    ) => {
        check_monom_prop!{ @build_quick_check $monom {$($acc)*} $($rests)* }
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
    mod power = Power for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod lex2 = Lex2 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod lex3 = Lex3 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod lex4 = Lex4 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod lex5 = Lex5 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod grevlex2 = Grevlex2 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn resp_total_deg(xs, ys) -> TestResult;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod grevlex3 = Grevlex3 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn resp_total_deg(xs, ys) -> TestResult;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod grevlex4 = Grevlex4 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn resp_total_deg(xs, ys) -> TestResult;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod grevlex5 = Grevlex5 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn resp_total_deg(xs, ys) -> TestResult;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod grlex2 = Grlex2 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn resp_total_deg(xs, ys) -> TestResult;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod grlex3 = Grlex3 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn resp_total_deg(xs, ys) -> TestResult;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod grlex4 = Grlex4 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn resp_total_deg(xs, ys) -> TestResult;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }

    mod grlex5 = Grlex5 for {
        fn mul_resp_ord(xs,ys,zs) -> bool;
        fn positive(xs) -> bool;
        fn var_decreasing() -> bool;
        fn resp_total_deg(xs, ys) -> TestResult;
        fn one_is_unit(xs) -> bool;
        fn mul_assoc(xs, ys, zs) -> bool;
        fn lcm_divisible(xs, ys) -> bool;
        fn lcm_least(xs, ys) -> bool;
    }
}
