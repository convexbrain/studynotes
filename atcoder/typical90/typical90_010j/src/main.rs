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

    let n: usize = spl.next().unwrap().parse().unwrap();

    dprintln!("{}", n);

    let mut ss = vec![(0, 0); n + 1];

    let mut ps = (0, 0);
    for i in 1..=n {
        let c: u32 = spl.next().unwrap().parse().unwrap();
        let p: u32 = spl.next().unwrap().parse().unwrap();

        dprintln!("{} {}", c, p);

        let ns = if c == 1 {(ps.0 + p, ps.1)} else {(ps.0, ps.1 + p)};
        dprintln!("{:?}", ns);

        ss[i] = ns;
        ps = ns;
    }

    let q: usize = spl.next().unwrap().parse().unwrap();

    dprintln!("{}", q);

    for _ in 0..q {
        let l: usize = spl.next().unwrap().parse().unwrap();
        let r: usize = spl.next().unwrap().parse().unwrap();

        dprintln!("{} {}", l, r);

        let a = ss[r].0 - ss[l - 1].0;
        let b = ss[r].1 - ss[l - 1].1;

        println!("{} {}", a, b);
    }

}
