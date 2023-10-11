use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;

macro_rules! dprintln {
    ( $($x:tt)* ) => {
        #[cfg(debug_assertions)]
        {
            print!("@{}:", line!());
            println!($($x)*);
        }
    };
}

fn gcd<N>(mut m: N, mut n: N) -> N
where N: Rem<Output=N> + Ord + Default + Copy
{
    if m < n {
        (m, n) = (n, m);
    }
    while n != N::default() {
        (m, n) = (n, m % n);
    }
    return m;
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let a: u64 = token.next().unwrap().parse().unwrap();
    let b: u64 = token.next().unwrap().parse().unwrap();

    dprintln!("{} {}", a, b);

    let g = gcd(a, b);
    let a_g = a / g;
    let b_g = b / g;

    let log10_l = (a_g as f64).log10() + (b_g as f64).log10() + (g as f64).log10();

    if log10_l >= 19.0 {
        println!("Large");
    } else {
        let l = a_g * b_g * g;
        if l > 1_000_000_000_000_000_000 {
            println!("Large");
        }
        else {
            println!("{}", l);
        }
    }
}
