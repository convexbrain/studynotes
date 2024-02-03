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
    let s: usize = token.next().unwrap().parse().unwrap();
    let m: usize = token.next().unwrap().parse().unwrap();
    let l: usize = token.next().unwrap().parse().unwrap();
    debug!(n, s, m, l);

    let mut min_money = usize::MAX;

    let smax = n / 6 + 1;
    for si in 0..=smax {
        if n > si * 6 {
            let nn = n - si * 6;
            let mmax = nn / 8 + 1;
            for mi in 0..=mmax {
                if nn > mi * 8 {
                    let nnn = nn - mi * 8;
                    let li = (nnn + 11) / 12;

                    let money = si * s + mi * m + li * l;
                    min_money = min_money.min(money);
                }
                else {
                    let money = si * s + mi * m;
                    min_money = min_money.min(money);
                }
            }
        }
        else {
            let money = si * s;
            min_money = min_money.min(money);
        }
    }

    println!("{min_money}");
}
