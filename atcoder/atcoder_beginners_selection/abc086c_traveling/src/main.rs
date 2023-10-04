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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: i32 = spl.next().unwrap().parse().unwrap();

    dprintln!("{}", n);

    let mut txy = (0, 0, 0);

    for _ in 0..n {
        let t: i32 = spl.next().unwrap().parse().unwrap();
        let x: i32 = spl.next().unwrap().parse().unwrap();
        let y: i32 = spl.next().unwrap().parse().unwrap();

        dprintln!("{} {} {}", t, x, y);

        let d = (txy.1 - x).abs() + (txy.2 - y).abs();

        if (txy.0 + d > t) || ((t - txy.0 - d) & 1 == 1) {
            println!("No");
            return;
        }

        txy = (t, x, y);
    }

    println!("Yes");
}
