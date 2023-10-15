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

    let mut n: u64 = token.next().unwrap().parse().unwrap();

    dprintln!("{}", n);

    let mut cnt = 0_u64;

    while n & 1 == 0 {
        n >>= 1;
        cnt += 1;
    }

    for k in 0.. {
        let kk = 2 * k + 3; // 3, 5, 7,...

        if kk * kk > n {
            break;
        }

        while n % kk == 0 {
            n = n / kk;
            cnt += 1;
        }
    }

    if n > 1 {
        cnt += 1;
    }

    dprintln!("{}", cnt);

    if cnt == 0 {
        println!("{}", 0);
    }
    else {
        let log_cnt = cnt.ilog2();

        dprintln!("{}", log_cnt);
    
        if (1_u64 << log_cnt) == cnt {
            println!("{}", log_cnt);
        }
        else {
            println!("{}", log_cnt + 1);
        }
    }
}
