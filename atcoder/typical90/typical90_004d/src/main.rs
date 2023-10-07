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

    let h: usize = spl.next().unwrap().parse().unwrap();
    let w: usize = spl.next().unwrap().parse().unwrap();

    dprintln!("{} {}", h, w);

    let mut a = vec![0; h * w];

    let mut cs = vec![0; w];
    let mut rs = vec![0; h];

    for r in 0..h {
        for c in 0..w {
            let v = spl.next().unwrap().parse().unwrap();

            a[r * w + c] = v;

            cs[c] += v;
            rs[r] += v;
        }
    }

    dprintln!("{:?}", a);
    dprintln!("{:?}", cs);
    dprintln!("{:?}", rs);

    for r in 0..h {
        for c in 0..w {
            let v = cs[c] + rs[r] - a[r * w + c];
            print!("{} ", v);
        }
        println!();
    }
    
}
