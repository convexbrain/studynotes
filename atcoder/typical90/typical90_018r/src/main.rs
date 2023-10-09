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

    let t: f64 = token.next().unwrap().parse().unwrap();
    let l: f64 = token.next().unwrap().parse().unwrap();
    let x: f64 = token.next().unwrap().parse().unwrap();
    let y: f64 = token.next().unwrap().parse().unwrap();
    let q: usize = token.next().unwrap().parse().unwrap();

    dprintln!("{} {} {} {} {}", t, l, x, y, q);

    for _ in 0.. q {
        let e: f64 = token.next().unwrap().parse().unwrap();

        dprintln!("{}", e);

        let theta = 2.0 * std::f64::consts::PI * e / t;
        let wy = -theta.sin() * l / 2.0;
        let wz = (1.0 - theta.cos()) * l / 2.0;

        dprintln!("{} {}", wy, wz);

        let d = x.hypot(y - wy);
        let angle = (wz / d).atan() / std::f64::consts::PI * 180.0;

        println!("{}", angle);
    }
}
