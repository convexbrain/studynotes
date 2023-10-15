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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();
    let q: usize = token.next().unwrap().parse().unwrap();

    dprintln!("{} {}", n, q);

    let mut d = vec![0; n - 1];

    let mut a0: i64 = token.next().unwrap().parse().unwrap();
    let mut e = 0_u64;

    dprintln!("{}", a0);

    for i in 1..n {
        let a: i64 = token.next().unwrap().parse().unwrap();

        dprintln!("{}", a);

        d[i - 1] = a - a0;
        e += a.abs_diff(a0);

        a0 = a;
    }

    dprintln!("{:?} {}", d, e);

    for _ in 0..q {
        let l: usize = token.next().unwrap().parse().unwrap();
        let r: usize = token.next().unwrap().parse().unwrap();
        let v: i64 = token.next().unwrap().parse().unwrap();

        dprintln!("{} {} {}", l, r, v);

        if l > 1 {
            e -= d[l - 2].abs() as u64;
            d[l - 2] += v;
            e += d[l - 2].abs() as u64;
        }
        if r < n {
            e -= d[r - 1].abs() as u64;
            d[r - 1] -= v;
            e += d[r - 1].abs() as u64;
        }

        dprintln!("{:?}", d);

        println!("{}", e);
    }
}
