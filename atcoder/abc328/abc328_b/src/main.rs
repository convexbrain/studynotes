use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::*, cmp::*,
    rc::*, cell::*, ops::Bound::*,
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

    let n: u32 = token.next().unwrap().parse().unwrap();

    let mut ans = 0;

    for m in 0..n {
        let mut m = m + 1;

        let d: u32 = token.next().unwrap().parse().unwrap();

        debug!(d);

        let mut k = m % 10;
        //debug!(k);
        while m > 0 {
            //debug!(m);
            if k != m % 10 {
                k = 0;
                break;
            }
            m /= 10;
        }
        debug!(k);

        if k == 0 {
            continue;
        }

        let mut kk = k;
        while kk <= d {
            debug!(kk);
            kk = kk * 10 + k;
            ans += 1;
        }
    }

    println!("{ans}");
}
