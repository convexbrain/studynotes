use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::{*, Bound::*}, cmp::*,
    str, rc::*, cell::*,
};

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ( $($x:tt)* ) => {};
}

#[cfg(debug_assertions)]
macro_rules! debug {
    () => {
        eprintln!("[@{}]", line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            ref tmp => {
                eprintln!("[@{}] {} = {:?}",
                    line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(debug!($val)),+,)
    };
}

//#############################################################################

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let d: u64 = token.next().unwrap().parse().unwrap();
    debug!(d);

    let fd = d as f64;
    let x_max = fd.sqrt().ceil() as u64;
    debug!(x_max);

    let mut ans = u64::MAX;
    for x in 0..=x_max {
        debug!(x);
        let fx = x as f64;
        //let fy = (fd - fx * fx).sqrt();
        let fy = (fd.sqrt() - fx).sqrt() * (fd.sqrt() + fx).sqrt();
        let y0 = fy.floor() as u64;
        let y1 = fy.ceil() as u64;
        debug!(y0, y1);

        let a0 = (x * x + y0 * y0).abs_diff(d);
        let a1 = (x * x + y1 * y1).abs_diff(d);
        ans = ans.min(a0.min(a1));
        if ans == 0 {
            break;
        }
    }

    println!("{ans}");
}
