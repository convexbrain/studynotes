use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::{collections::*, ops::*};
use std::{rc::*, cell::*, ops::Bound::*};

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

    debug!(n, k);

    let a: Vec<u32> = (0..n).map(|_| token.next().unwrap().parse().unwrap()).collect();

    debug!(a);

    let mut l = 0;
    let mut r = 0;
    let mut len_max = 0;
    let mut cnt = BTreeMap::new();

    loop {
        debug!(l, r);

        if cnt.len() <= k {
            let len = r - l;
            len_max = len_max.max(len);
            debug!(len, len_max);

            if r < n {
                cnt.insert(a[r], cnt.get(&a[r]).unwrap_or(&0_usize) + 1);
                r += 1;
            }
            else {
                break;
            }
        }
        else {
            let c = cnt[&a[l]];
            if c > 1 {
                let cc = cnt.get_mut(&a[l]).unwrap();
                *cc -= 1;
            }
            else {
                cnt.remove(&a[l]);
            }
            l += 1;
        }
    }
    
    println!("{}", len_max);
}
