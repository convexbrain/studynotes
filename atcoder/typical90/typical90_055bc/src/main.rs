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

    let n: usize = token.next().unwrap().parse().unwrap();
    let p: u64 = token.next().unwrap().parse().unwrap();
    let q: u64 = token.next().unwrap().parse().unwrap();

    dprintln!("{} {} {}", n, p, q);

    let a: Vec<u64> = (0..n).map(|_| {
        let v = token.next().unwrap().parse().unwrap();
        v
    }).collect();

    dprintln!("{:?}", a);

    let mut cnt = 0;
    for i0 in 0..(n - 4) {
        let r0 = a[i0] % p;
        for i1 in (i0 + 1)..(n - 3) {
            let r1 = a[i1] % p;
            for i2 in (i1 + 1)..(n - 2) {
                let r2= a[i2] % p;
                for i3 in (i2 + 1)..(n - 1) {
                    let r3= a[i3] % p;
                    for i4 in (i3 + 1)..n {
                        let r4= a[i4] % p;

                        let r = (r0 * r1) % p;
                        let r = (r2 * r) % p;
                        let r = (r3 * r) % p;
                        let r = (r4 * r) % p;

                        dprintln!("{} {} {} {} {} - {}", i0, i1, i2, i3, i4, r);

                        if r == q {
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", cnt);
}
