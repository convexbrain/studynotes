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
    let k: u32 = spl.next().unwrap().parse().unwrap();

    dprintln!("{} {}", n, k);

    let a: Vec<u32> = (0..n).map(|_| {
        spl.next().unwrap().parse().unwrap()
    }).collect();

    dprintln!("{:?}", a);

    let mut d = 0;

    for i in 0..n {
        let b: u32 = spl.next().unwrap().parse().unwrap();

        dprintln!("{}", b);

        d += b.abs_diff(a[i]);
        dprintln!("{}", d);
    }

    if d <= k && ((k - d) & 1) == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
