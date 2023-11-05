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

fn check(root: &mut HashMap<u64, HashSet<(u64, u64)>>, x: u64) {
    if !root.contains_key(&x) {
        let mut set = HashSet::new();
        let r_max = (x as f64).sqrt().ceil() as u64;
        for r in 1..=r_max {
            if x % r == 0 && r <= x / r {
                set.insert((r, x / r));
            }
        }
        root.insert(x, set);
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let k: u64 = token.next().unwrap().parse().unwrap();

    debug!(k);

    let mut root = HashMap::new();
    let mut sol = HashSet::new();

    check(&mut root, k);
    let kpq = root[&k].clone();

    for (kp, kq) in kpq.iter() {
        check(&mut root, *kp);
        check(&mut root, *kq);

        for (kpp, kpq) in root[&kp].iter() {
            let mut tri = vec![*kpp, *kpq, *kq];
            tri.sort();
            sol.insert((tri[0], tri[1], tri[2]));
        }

        for (kqp, kqq) in root[&kq].iter() {
            let mut tri = vec![*kqp, *kqq, *kp];
            tri.sort();
            sol.insert((tri[0], tri[1], tri[2]));
        }
    }

    debug!(sol);
    let s = sol.len();
    
    println!("{s}");
}
