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
    let a: Vec<u64> = (0..n).map(|_| token.next().unwrap().parse().unwrap()).collect();

    dprintln!("{} {:?}", n, a);

    let sum: u64 = a.iter().sum();

    dprintln!("{}", sum);

    if sum % 10 > 0 {
        println!("No");
        return;
    }

    let tgt = sum / 10;

    dprintln!("{}", tgt);

    let aa = a.repeat(2);

    let mut l = 0_usize;
    let mut r = 1_usize;
    let mut s = a[0];

    loop {
        if l == n {
            println!("No");
            return;
        }
        else if s == tgt {
            println!("Yes");
            return;
        }
        else if s < tgt {
            s += aa[r];
            r += 1;
        }
        else {
            s -= aa[l];
            l += 1;
        }
    }

    /*
    let mut m = BTreeMap::new();

    for (i, val) in a.iter().enumerate() {
        if *val == tgt {
            println!("Yes");
            return;
        }

        if *val < tgt {
            m.insert(i, *val);
        }
    }

    for r in 1..n {
        dprintln!("{:?}", m);

        if m.len() == 0 {
            println!("No");
            return;
        }

        let mut yes = false;
        m.retain(|k, v| {
            let nv = *v + a[(k + r as usize) % (n as usize)];

            if nv == tgt {
                yes = true;
            }

            *v = nv;
            nv < tgt
        });

        if yes {
            println!("Yes");
            return;
        }
    }

    println!("No");
     */

}
