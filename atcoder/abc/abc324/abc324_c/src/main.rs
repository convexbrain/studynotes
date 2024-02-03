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
    let td = token.next().unwrap(); // &str
    let b: Vec<&str> = (0..n).map(|_| token.next().unwrap()).collect();

    dprintln!("{} {} {:?}", n, td, b);

    let mut a = vec![0; n];

    for i in 0..n {
        let l = td.len();

        if td == b[i] {
            a[i] = 1;
        }
        else if l + 1 == b[i].len() {
            let vtd: Vec<char> = td.chars().collect();
            let vb: Vec<char> = b[i].chars().collect();

            let mut j = 0;
            while j < l && vtd[j] == vb[j] {
                j += 1;
            }
            //dprintln!("{}", j);

            if j < l {
                //dprintln!("{} {}", &td[j..], &b[i][(j + 1)..]);
                if td[j..] == b[i][(j + 1)..] {
                    a[i] = 1;
                }
            }
            else {
                a[i] = 1;
            }
        }
        else if l == b[i].len() + 1 {
            dprintln!("{} {}", td, b[i]);
            let vtd: Vec<char> = td.chars().collect();
            let vb: Vec<char> = b[i].chars().collect();

            let mut j = 0;
            while j < l - 1 && vtd[j] == vb[j] {
                j += 1;
            }
            //dprintln!("{}", j);

            if j < l - 1 {
                //dprintln!("{} {}", &td[(j + 1)..], &b[i][j..]);
                if td[(j + 1)..] == b[i][j..] {
                    a[i] = 1;
                }
            }
            else {
                a[i] = 1;
            }
        }
        else if l == b[i].len() {
            let vtd: Vec<char> = td.chars().collect();
            let vb: Vec<char> = b[i].chars().collect();

            let mut j = 0;
            while j < l && vtd[j] == vb[j] {
                j += 1;
            }
            //dprintln!("{}", j);

            if j < l {
                //dprintln!("{} {}", &td[(j + 1)..], &b[i][(j + 1)..]);
                if td[(j + 1)..] == b[i][(j + 1)..] {
                    a[i] = 1;
                }
            }
            else {
                a[i] = 1;
            }
        }
    }

    println!("{}", a.iter().sum::<u32>());

    for i in 0..n {
        if a[i] == 1 {
            print!("{} ", i + 1);
        }
    }
    println!();
}
