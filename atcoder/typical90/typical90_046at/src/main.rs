use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;

macro_rules! dprintln {
    ( $($x:tt)* ) => {
        #[cfg(debug_assertions)]
        {
            print!("@{}:", line!());
            println!($($x)*);
        }
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();

    let mut a = BTreeMap::new();
    for _ in 0..n {
        let x: u32 = token.next().unwrap().parse().unwrap();
        let x = x % 46;

        a.entry(x)
         .and_modify(|e| {*e += 1})
         .or_insert(1_u64);
    }

    let mut b = BTreeMap::new();
    for _ in 0..n {
        let x: u32 = token.next().unwrap().parse().unwrap();
        let x = x % 46;

        b.entry(x)
         .and_modify(|e| {*e += 1})
         .or_insert(1_u64);
    }

    let mut c = BTreeMap::new();
    for _ in 0..n {
        let x: u32 = token.next().unwrap().parse().unwrap();
        let x = x % 46;

        c.entry(x)
         .and_modify(|e| {*e += 1})
         .or_insert(1_u64);
    }

    dprintln!("{} {:?} {:?} {:?}", n, a, b, c);

    let mut cnt = 0;
    for (qa, ca) in a.iter() {
        for (qb, cb) in b.iter() {
            for (qc, cc) in c.iter() {
                if (qa + qb + qc) % 46 == 0 {
                    cnt += ca * cb * cc;
                }
            }
        }
    }

    println!("{}", cnt);
}
