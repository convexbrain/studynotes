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

    let n: usize = token.next().unwrap().parse().unwrap();
    let q: usize = token.next().unwrap().parse().unwrap();
    //let s = token.next().unwrap(); // &str
    let c = token.next().unwrap().as_bytes(); // &[u8]
    //let d = token.next().unwrap().as_bytes().to_vec(); // Vec<u8>
    //let d = String::from_utf8(d).unwrap(); // Vec<u8> -> String

    let mut v = vec![0; n];
    let mut vv = 0;
    for i in 1..n {
        if c[i] == c[i - 1] {
            vv += 1;
        }
        v[i] = vv;
    }
    debug!(v);

    for _ in 0..q {
        let l: usize = token.next().unwrap().parse().unwrap();
        let r: usize = token.next().unwrap().parse().unwrap();

        debug!(l, r);

        let l = l - 1;
        let r = r - 1;

        let ans = v[r] - v[l];
        println!("{ans}");
    }
    
}
