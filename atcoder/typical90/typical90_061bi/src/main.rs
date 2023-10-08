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

    let q: usize = token.next().unwrap().parse().unwrap();

    dprintln!("{}", q);

    let mut deq = VecDeque::new();

    for _ in 0..q {
        let t: usize = token.next().unwrap().parse().unwrap();
        let x: usize = token.next().unwrap().parse().unwrap();

        dprintln!("{} {}", t, x);

        match t {
            1 => {
                deq.push_front(x);
            },
            2 => {
                deq.push_back(x);
            },
            3 => {
                let y = deq.get(x - 1).unwrap();
                println!("{}", y);
            },
            _ => {
                panic!();
            }
        }


        deq[0];
    }
}
