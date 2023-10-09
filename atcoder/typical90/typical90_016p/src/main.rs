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

    let n: u32 = token.next().unwrap().parse().unwrap();
    let mut abc: Vec<u32> = (0..3).map(|_| token.next().unwrap().parse().unwrap()).collect();

    abc.sort();
    abc.reverse();

    dprintln!("{} {:?}", n, abc);

    let mut min_i = u32::MAX;

    let m0 = n / abc[0];

    for i0 in (0..=m0).rev() {
        let n0 = n - abc[0] * i0;

        let m1 = n0 / abc[1];

        for i1 in (0..=m1).rev() {
            let n1 = n0 - abc[1] * i1;

            if n1 % abc[2] == 0 {
                let i2 = n1 / abc[2];

                let i = i0 + i1 + i2;
                min_i = min_i.min(i);
            }
        }
    }

    println!("{}", min_i);
}
