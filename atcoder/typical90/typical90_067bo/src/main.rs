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

    let mut n: Vec<u8> = token.next().unwrap().bytes().collect();
    let k: usize = token.next().unwrap().parse().unwrap();

    for d in n.iter_mut() {
        *d = *d - b'0';
    }

    dprintln!("{:?} {}", n, k);


    for _ in 0..k {
        let mut x = 0u64;

        for d in n.iter() {
            x *= 8;
            x += *d as u64;
        }
    
        dprintln!("{}", x);
    
        for dd in 0..n.len() {
            let p = (x % 9) as u8;
            x /= 9;
    
            let p = if p == 8 {5} else {p};

            let d = n.len() - 1 - dd;
            n[d] = p;
        }
    
        dprintln!("{:?}", n);
    }


    let mut first = false;
    for d in n {
        if first || d > 0 {
            first = true;
            print!("{}", d);
        }
    }
    if !first {
        println!("0");
    }
    else {
        println!("");
    }
}
