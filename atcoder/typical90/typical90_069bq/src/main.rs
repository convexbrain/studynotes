use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;
use std::cmp::*;

macro_rules! dprintln {
    ( $($x:tt)* ) => {
        #[cfg(debug_assertions)]
        {
            eprint!("@{}:", line!());
            eprintln!($($x)*);
        }
    };
}

fn mod_mul<N>(x: N, y: N, m: N) -> N
where N: Mul<Output=N> + Rem<Output=N>
{
    (x * y) % m
}

fn mod_pow<N>(mut x: N, mut p: N, m: N) -> N
where N: Default + Ord + BitAnd<Output=N> + ShrAssign + Mul<Output=N> + Rem<Output=N> + SubAssign + Copy + Div<Output=N>
{
    let zero = N::default();
    let one = m / m;

    if p == zero {
        return one;
    }

    let mut k = one;

    while p > one {
        if p & one == zero {
            x = mod_mul(x, x, m);
            p >>= one;
        }
        else {
            k = mod_mul(k, x, m);
            p -= one;
        }
    }
    mod_mul(k, x, m)
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: u64 = token.next().unwrap().parse().unwrap();
    let k: u64 = token.next().unwrap().parse().unwrap();

    dprintln!("{} {}", n, k);

    let m = 1000000007;

    match (n, k) {
        (1, 1) => {
            println!("1");
            return;
        },
        (1, 2) => {
            println!("2");
            return;
        },
        (1, kk) => {
            println!("{}", kk);
            return;
        },
        (2, 1) => {
            println!("0");
            return;
        },
        (2, 2) => {
            println!("2");
            return;
        },
        (2, kk) => {
            println!("{}", mod_mul(kk, kk - 1, m));
            return;
        },
        (_, kk) => {
            if kk < 3 {
                println!("0");
                return;
            }
        },
    }

    assert!(n >= 3);
    assert!(k >= 3);

    let perm_k_3 = mod_mul(k, k - 1, m);
    let perm_k_3 = mod_mul(perm_k_3, k - 2, m);

    if n > 3 {
        let k_2_pow = mod_pow(k - 2, n - 3, m);

        println!("{}", mod_mul(perm_k_3, k_2_pow, m));
    }
    else {
        println!("{}", perm_k_3);
    }
}
