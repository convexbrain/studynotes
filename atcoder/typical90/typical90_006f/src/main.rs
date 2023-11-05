use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::*,
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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();
    let k: usize = token.next().unwrap().parse().unwrap();
    let s: Vec<u8> = token.next().unwrap().bytes().collect();

    debug!(n, k, s);

    let mut c = vec![vec![n; (b'z' - b'a' + 1) as usize]; n];

    for (i, val) in s.iter().enumerate().rev() {
        if i < n - 2 {
            for j in 0..c[i].len() {
                c[i][j] = c[i + 1][j];
            }
        }
        c[i][(val - b'a') as usize] = i;
    }
    debug!(c);

    let mut ans = Vec::new();

    let mut pos = 0;
    for i in 0..k {
        debug!(pos);
        for j in 0..c[pos].len() {
            if c[pos][j] < n - k + i + 1 {
                debug!(j);
                let ch = b'a' + j as u8;
                ans.push(ch);
                pos = if ch == s[pos] {pos + 1} else {c[pos][j] + 1};
                break;
            }
        }
    }

    let ans = String::from_utf8(ans).unwrap();
    
    println!("{ans}");
}
