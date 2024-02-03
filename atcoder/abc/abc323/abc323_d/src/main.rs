use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;

macro_rules! dprintln {
    ( $($x:tt)* ) =>
    {
        #[cfg(debug_assertions)]
        {
            print!("[{}]", line!());
            println!($($x)*);
        }
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: u64 = spl.next().unwrap().parse().unwrap();

    dprintln!("{}", n);

    let mut smap = BTreeMap::new();

    for _ in 0..n {
        let s: u64 = spl.next().unwrap().parse().unwrap();
        let c: u64 = spl.next().unwrap().parse().unwrap();

        smap.insert(s, c);

        dprintln!("{} {}", s, c);
    }
    dprintln!("{:?}", smap);

    loop {
        let mut sel_s = 0;
        for (s, c) in smap.iter() {
            if c >= &2 {
                sel_s = *s;
                break;
            }
        }

        if sel_s == 0 {
            break;
        }

        let c = smap.get(&sel_s).unwrap_or(&0);
        let log2c = c.ilog2();
        dprintln!("{} {}", c, log2c);

        let nc = c - (1 << log2c);
        smap.insert(sel_s, nc);
        let ns = sel_s * (1 << log2c);
        let nc = smap.get(&ns).unwrap_or(&0) + 1;
        smap.insert(ns, nc);
        dprintln!("{:?}", smap);
    }

    let t = smap.iter().fold(0, |a, x| a + x.1);

    println!("{}", t);
}
