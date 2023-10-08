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

    dprintln!("{}", n);

    if n & 1 == 1 {return;}

    let mut d = vec![BTreeSet::new(); n / 2]; // 2, 4, 6, ...
    d[0].insert("()".to_string());

    for i in 1..(n / 2) {
        let l = i * 2 + 2;

        let mut nd = BTreeSet::new();

        for p in d[i - 1].iter() {
            let np = format!("({})", p);
            assert_eq!(np.len(), l);

            nd.insert(np);
        }

        for j in 0..i {
            let k = i - 1 - j;
            dprintln!("{} {} {}", i, j, k);

            for p in d[j].iter() {
                for p2 in d[k].iter() {
                    let np = format!("{}{}", p, p2);
                    assert_eq!(np.len(), l);
                    
                    nd.insert(np);
                }
            }
        }

        d[i] = nd;
    }

    for p in d[n / 2 - 1].iter() {
        println!("{}", p);
    }
}
