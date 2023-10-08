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
    let m: usize = token.next().unwrap().parse().unwrap();

    dprintln!("{} {}", n, m);

    let mut c = vec![0; n];

    for _ in 0..m {
        let a: usize = token.next().unwrap().parse().unwrap();
        let b: usize = token.next().unwrap().parse().unwrap();
    
        dprintln!("{} {}", a, b);

        let l = if a > b {a} else {b};
        c[l - 1] += 1;
    }

    dprintln!("{:?}", c);

    let r = c.iter().fold(0, |a, x| if x == &1 {a + 1} else {a});

    println!("{}", r);
}
