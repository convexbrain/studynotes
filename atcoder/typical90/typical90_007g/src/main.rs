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

    dprintln!("{}", n);

    let mut a: Vec<u32> = Vec::from_iter((0..n).map(|_| {
        token.next().unwrap().parse().unwrap()}
    ));
    a.sort();

    dprintln!("{:?}", a);

    let q: usize = token.next().unwrap().parse().unwrap();

    dprintln!("{}", q);

    for _ in 0..q {
        let b: u32 = token.next().unwrap().parse().unwrap();

        dprintln!("{}", b);

        let m = a.binary_search(&b);

        match m {
            Ok(_) => {
                println!("0");
            },
            Err(i) => {
                if i == 0 {
                    println!("{}", b.abs_diff(a[i]));
                }
                else if i == a.len() {
                    println!("{}", b.abs_diff(a[i - 1]));
                }
                else {
                    println!("{}", b.abs_diff(a[i]).min(
                        b.abs_diff(a[i - 1])
                    ));
                }
            }
        }
    }
}
