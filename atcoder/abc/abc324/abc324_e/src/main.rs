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
    let t: Vec<char> = token.next().unwrap().chars().collect();
    let s: Vec<&str> = (0..n).map(|_| token.next().unwrap()).collect();

    dprintln!("{} {:?} {:?}", n, t, s);

    let mut cnt = 0;
    for i in 0..n {
        for j in 0..n {
            let ss = s[i].to_string() + s[j];
            //dprintln!("{}", ss);

            let mut p = 0;
            for k in 0..t.len() {
                dprintln!("{} {}", k, t[k]);
                if p < ss.len() {
                    if let Some(idx) = ss[p..].find(t[k]) {
                        p = idx + 1;
                    }
                    else {
                        p = ss.len();
                        break;
                    }
                }
                else {
                    p = ss.len();
                    break;
                }
            }

            if p < ss.len() {
                cnt += 1;
                dprintln!("{} {}", ss, cnt);
            }

        }
    }

    println!("{}", cnt);
}
