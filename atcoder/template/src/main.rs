use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;
use std::cmp::*;

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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let a = token.next().unwrap(); // &str
    let b = token.next().unwrap().to_string(); // String
    let c = token.next().unwrap().as_bytes(); // &[u8]
    let d = token.next().unwrap().as_bytes().to_vec(); // Vec<u8>
    let n: usize = token.next().unwrap().parse().unwrap();

    let d = String::from_utf8(d).unwrap(); // Vec<u8> -> String

    debug!(a, b, c, d, n);
    
    println!("{}", buf);
}
