use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;
use std::cmp::*;
use std::ops::Bound::*;

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
    let m: u32 = token.next().unwrap().parse().unwrap();

    debug!(n, m);

    let mut map = BTreeMap::new();

    for _ in 0..n {
        let a: u32 = token.next().unwrap().parse().unwrap();

        debug!(a);

        map.insert(a, map.get(&a).unwrap_or(&0) + 1_u32);
    }

    debug!(map);

    let a_min = *map.first_key_value().unwrap().0;
    let a_max = *map.last_key_value().unwrap().0;

    let range = map.range(a_min..(a_min + m));

    let mut al = a_min;
    let mut ar = a_min;
    let mut cnt_max = range.fold(0, |acc, x| {
        ar = *x.0;
        acc + x.1
    });

    debug!(al, ar, cnt_max);

    let mut cnt = cnt_max;
    loop {
        cnt -= map[&al];

        if let Some((&n_al, _)) = map.range((Excluded(&al), Included(&a_max))).next() {
            al = n_al;
        }
        else {
            println!("{}", cnt_max);
            return;
        }

        loop {
            if let Some((&n_ar, c)) = map.range((Excluded(&ar), Included(&a_max))).next() {
                if al + m > n_ar {
                    ar = n_ar;
                    cnt += c;
                }
                else {
                    cnt_max = cnt_max.max(cnt);
                    break;
                }
            }
            else {
                println!("{}", cnt_max);
                return;
            }
        }
    }
}
