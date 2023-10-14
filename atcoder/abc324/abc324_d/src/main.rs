use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;
use std::cmp::*;

macro_rules! dprintln {
    ( $($x:tt)* ) => {
        #[cfg(debug_assertions)]
        {
            eprint!("@{}:", line!());
            eprintln!($($x)*);
        }
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();
    let s = token.next().unwrap().as_bytes(); // &[u8]

    dprintln!("{} {:?}", n, s);

    let mut sm = vec![0; 10];

    for d in s {
        let v = d - b'0';
        sm[v as usize] += 1;
    }

    dprintln!("{:?}", sm);
    
    let mut minv = 0_u64;
    for (i, val) in sm.iter().enumerate() {
        for _ in 0..*val {
            minv = minv * 10 + i as u64;
        }
    }
    dprintln!("{}", minv);
    let minr = (minv as f64).sqrt().floor() as u64;
    dprintln!("{}", minr);

    let mut maxv = 0_u64;
    for (i, val) in sm.iter().enumerate().rev() {
        for _ in 0..*val {
            maxv = maxv * 10 + i as u64;
        }
    }
    dprintln!("{}", maxv);
    let maxr = (maxv as f64).sqrt().ceil() as u64;
    dprintln!("{}", maxr);

    //

    let mut cnt = 0;
    for r in minr..=maxr {
        let mut v = r * r;

        let mut sm2 = sm.clone();
        let mut bad = false;

        for _ in 0..s.len() {
            let q = (v % 10) as usize;
            v /= 10;

            if sm2[q] > 0 {
                sm2[q] -= 1;
            }
            else {
                bad = true;
                break;
            }
        }

        if !bad {
            dprintln!("{}", r * r);
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
