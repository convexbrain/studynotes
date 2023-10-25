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

    let n: usize = token.next().unwrap().parse().unwrap();
    let s = token.next().unwrap();

    debug!(n, s);

    let t = "atcoder";
    let p = 1_000_000_007;
    
    debug!(t, p);

    let mut ss = Vec::new();
    let mut sc = Vec::new();
    {
        let mut pc = '0';
        let mut cnt = 0_u32;
        for c in s.chars() {
            if c == pc {
                cnt += 1;
            }
            else {
                if n > 0 {
                    ss.push(pc);
                    sc.push(cnt);
                }
                if t.contains(c) {
                    pc = c;
                    cnt = 1;
                }
                else {
                    pc = '0';
                    cnt = 0;
                }
            }
        }
        if cnt > 0 {
            ss.push(pc);
            sc.push(cnt);
        }
    }
    let ss = String::from_iter(ss);
    debug!(ss);
    debug!(sc);

    let s = ss.as_str();

    let mut h = BTreeMap::new();
    let mut h1 = BTreeMap::new();

    for (i, tc) in t.chars().enumerate() {
        debug!(tc);
        for (pos, _) in s.match_indices(tc) {
            debug!(pos);

            if i == 0 {
                h.insert(pos, sc[pos]);
            }
            else {
                let score = h1.range(0..pos).fold(0, |acc, x| (acc + x.1) % p);
                let score = (score as u64 * sc[pos] as u64) % (p as u64);
                let score = score as u32;
                h.insert(pos, score);
            }
        }

        debug!(h);

        (h, h1) = (h1, h);
        h.clear();
    }

    let s = h1.iter().fold(0, |acc, x| (acc + x.1) % p);

    println!("{}", s);
}
