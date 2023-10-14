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
    let k: usize = token.next().unwrap().parse().unwrap();

    let mut ba = BinaryHeap::from_iter(
        (0..n).map(|_| {
            let a = token.next().unwrap().parse::<u32>().unwrap();
            let b = token.next().unwrap().parse::<u32>().unwrap();
            (b, a)
        })
    );

    dprintln!("{} {} {:?}", n, k, ba);

    let mut score = 0_u64;
    for _ in 0..k {
        let mut m = ba.pop().unwrap();

        score += m.0 as u64;
        if m.1 > 0 {
            m = (m.1 - m.0, 0);
        }
        else {
            m = (0, 0);
        }

        ba.push(m);

        dprintln!("{} {:?}", score, ba);
    }
    println!("{}", score);
}
