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
    let l: usize = token.next().unwrap().parse().unwrap();

    dprintln!("{} {}", n, l);

    let mut a = vec![0; n + 1];

    for i in 0..=n {
        if i < 1 {
            a[i] = 1;
        }
        else if i < l {
            a[i] = a[i - 1];
        }
        else {
            a[i] = (a[i - 1] + a[i - l]) % 1000000007;
        }

        dprintln!("{:?}", a);
    }

    println!("{}", a[n]);
}
