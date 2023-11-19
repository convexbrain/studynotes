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

    let n: usize = token.next().unwrap().parse().unwrap();
    let q: usize = token.next().unwrap().parse().unwrap();

    let mut rot_x_max = i32::MIN;
    let mut rot_y_max = i32::MIN;
    let mut rot_x_min = i32::MAX;
    let mut rot_y_min = i32::MAX;

    let rot_xy: Vec<(i32, i32)> = (0..n).map(|_| {
        let x: i32 = token.next().unwrap().parse().unwrap();
        let y: i32 = token.next().unwrap().parse().unwrap();
        let rot_x = x - y;
        let rot_y = x + y;
        rot_x_max = rot_x_max.max(rot_x);
        rot_y_max = rot_y_max.max(rot_y);
        rot_x_min = rot_x_min.min(rot_x);
        rot_y_min = rot_y_min.min(rot_y);
        (rot_x, rot_y)
    }).collect();
    debug!(rot_xy);

    for _ in 0..q {
        let qi: usize = token.next().unwrap().parse().unwrap();
        debug!(qi);
        let qi = qi - 1;

        let mut v = Vec::from([
            rot_x_max.abs_diff(rot_xy[qi].0),
            rot_y_max.abs_diff(rot_xy[qi].1),
            rot_x_min.abs_diff(rot_xy[qi].0),
            rot_y_min.abs_diff(rot_xy[qi].1),
        ]);
        v.sort();

        let ans = v[3];
        println!("{ans}");
    }
}
