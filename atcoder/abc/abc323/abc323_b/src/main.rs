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

    let mut wins = Vec::new();

    for i in 0..n {
        let s = spl.next().unwrap();
        let w = s.match_indices("o").fold(0, |a, _| a + 1);

        dprintln!("{} {}", s, w);

        wins.push((w, i + 1));
    }

    dprintln!("{:?}", wins);

    wins.sort_by(|a, b| b.0.cmp(&a.0));

    dprintln!("{:?}", wins);

    for i in 0..n {
        print!("{} ", wins[i].1);
    }
    println!();

}
