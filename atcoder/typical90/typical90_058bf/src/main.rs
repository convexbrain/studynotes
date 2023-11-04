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

    let n: u32 = token.next().unwrap().parse().unwrap();
    let k: u64 = token.next().unwrap().parse().unwrap();

    debug!(n, k);

    let mut map = HashMap::<u32, (u64, u32)>::new();

    let mut x = n;
    let mut c = None;
    for i in 0..k {
        if let Some(&i_z) = map.get(&x) {
            c = Some((i, i_z.0));
            break;
        }
        let mut xx = x;
        let mut y = 0;
        while xx > 0 {
            y += xx % 10;
            xx /= 10;
        }
        let z = (x + y) % 100000;

        map.insert(x, (i, z));

        x = z;
    }

    debug!(x, c);

    if let Some((i, i0)) = c {
        let rest = k - i;
        let loopm = i - i0;
        let cnt = rest % loopm;
        for _ in 0..cnt {
            x = map[&x].1;
        }
    }

    println!("{}", x);
}
