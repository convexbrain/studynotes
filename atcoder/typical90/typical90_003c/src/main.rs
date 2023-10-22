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

    debug!(n);

    let mut city = vec![HashSet::new(); n];

    for _ in 0..(n - 1) {
        let a: usize = token.next().unwrap().parse().unwrap();
        let b: usize = token.next().unwrap().parse().unwrap();
        let a = a - 1;
        let b = b - 1;
    
        debug!(a, b);

        city[a].insert(b);
        city[b].insert(a);
    }

    let mut que = VecDeque::new();
    let mut visited = HashSet::new();
    let mut s_max = (0, 0);

    for _ in 0..2 {
        visited.clear();
        que.push_back((s_max.0, 0));
        while let Some((qc, qs)) = que.pop_front() {
            visited.insert(qc);
            s_max = (qc, qs);
            debug!(s_max);
            for i in city[qc].iter() {
                if !visited.contains(i) {
                    que.push_back((*i, qs + 1));
                }
            }
        }
        debug!(s_max);
    }
    
    println!("{}", s_max.1 + 1);
}
