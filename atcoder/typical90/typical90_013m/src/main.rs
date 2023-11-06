use std::{prelude::rust_2021::*, cmp::Reverse};
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
    let m: usize = token.next().unwrap().parse().unwrap();

    debug!(n, m);

    let mut g = vec![HashSet::new(); n];

    for _ in 0..m {
        let a: usize = token.next().unwrap().parse().unwrap();
        let b: usize = token.next().unwrap().parse().unwrap();
        let c: u32 = token.next().unwrap().parse().unwrap();

        debug!(a, b, c);

        let a = a - 1;
        let b = b - 1;

        g[a].insert((b, c));
        g[b].insert((a, c));
    }
    
    debug!(g);

    let mut from1n = vec![(u32::MAX, u32::MAX); n];

    let mut q = BinaryHeap::new();

    q.push((Reverse(0), 0));
    while let Some((dist, city)) = q.pop() {
        if from1n[city].0 > dist.0 {
            from1n[city].0 = dist.0;

            for (ncity, ndist) in g[city].iter() {
                q.push((Reverse(dist.0 + ndist), *ncity));
            }
        }
    }

    q.push((Reverse(0), n - 1));
    while let Some((dist, city)) = q.pop() {
        if from1n[city].1 > dist.0 {
            from1n[city].1 = dist.0;

            for (ncity, ndist) in g[city].iter() {
                q.push((Reverse(dist.0 + ndist), *ncity));
            }
        }
    }

    debug!(from1n);

    for d in from1n.iter() {
        let ds = d.0 + d.1;
        println!("{ds}");
    }
}
