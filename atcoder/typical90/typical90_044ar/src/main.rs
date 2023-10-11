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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();
    let q: usize = token.next().unwrap().parse().unwrap();

    let mut a: Vec<u32> = (0..n).map(|_| token.next().unwrap().parse().unwrap()).collect();

    dprintln!("{} {} {:?}", n, q, a);

    let mut shift = 0;
    for _ in 0..q {
        let t: usize = token.next().unwrap().parse().unwrap();
        let x: usize = token.next().unwrap().parse().unwrap();
        let y: usize = token.next().unwrap().parse().unwrap();

        dprintln!("{} {} {}", t, x, y);

        match t {
            1 => {
                let ax = a[(x - 1 + shift) % n];
                let ay = a[(y - 1 + shift) % n];
                a[(x - 1 + shift) % n] = ay;
                a[(y - 1 + shift) % n] = ax;
            },
            2 => {
                shift = (shift + n - 1) % n;
            },
            3 => {
                let ax = a[(x - 1 + shift) % n];
                println!("{}", ax);
            },
            _ => {panic!();},
        }
    }
}
