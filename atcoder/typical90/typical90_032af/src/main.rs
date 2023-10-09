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

    let mut a = vec![vec![0; n]; n];
    for r in 0..n {
        for c in 0..n {
            a[r][c] = token.next().unwrap().parse().unwrap();
        }
    }

    let m: usize = token.next().unwrap().parse().unwrap();

    let mut xy = HashSet::new();
    for _ in 0..m {
        let x: u8 = token.next().unwrap().parse().unwrap();
        let y: u8 = token.next().unwrap().parse().unwrap();

        xy.insert((x - 1, y - 1));
    }

    dprintln!("{} {:?} {} {:?}", n, a, m, xy);

    //

    let mut min_t = u32::MAX;
    let mut s = vec![-1_i8; n];
    let mut j = 0;

    loop {
        s[j] += 1;

        if s[j] >= n as i8 {
            s[j] = -1;
            if j == 0 {
                break;
            }
            else {
                j -= 1;
                continue;
            }
        }
        else {
            if j > 0 {
                if let Some(_) = s[0..j].iter().find(|&&x| x == s[j]) {
                    continue;
                }
                if xy.contains(&(s[j] as u8, s[j - 1] as u8)) {
                    continue;
                }
                if xy.contains(&(s[j - 1] as u8, s[j] as u8)) {
                    continue;
                }
            }

            if j >= n - 1 {
                dprintln!("{:?}", s);

                let t = s.iter().enumerate().fold(0, |acc, (j, i)| {
                    acc + a[*i as usize][j]
                });

                min_t = min_t.min(t);

                dprintln!("{}", t);
            }
            else {
                j += 1;
            }
        }
    }

    if min_t < u32::MAX {
        println!("{}", min_t);
    }
    else {
        println!("-1");
    }
}
