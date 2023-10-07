use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;

macro_rules! dprintln {
    ( $($x:tt)* ) =>
    {
        #[cfg(debug_assertions)]
        {
            print!("[{}]", line!());
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

    loop {
        if n == N::default() {
            return m;
        }

        (m, n) = (n, m % n);
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let a: u64 = spl.next().unwrap().parse().unwrap();
    let b: u64 = spl.next().unwrap().parse().unwrap();
    let c: u64 = spl.next().unwrap().parse().unwrap();

    dprintln!("{} {} {}", a, b, c);

    let g = gcd(gcd(a, b), c);

    dprintln!("{}", g);

    let ac = a / g - 1;
    let bc = b / g - 1;
    let cc = c / g - 1;

    let cut = ac + bc + cc;

    println!("{}", cut);
}
