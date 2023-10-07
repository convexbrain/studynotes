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
    let m: usize = spl.next().unwrap().parse().unwrap();

    dprintln!("{} {}", n, m);

    let mut a: Vec<(u32, usize)> = Vec::from_iter(
        (0..m).map(|i|
            (spl.next().unwrap().parse().unwrap(), i)
        )
    );

    dprintln!("{:?}", a);

    let mut s = vec![(0, vec![false; m]); n];
    let mut max_score = 0;

    for ni in 0..n {
        let oxs = spl.next().unwrap();
        let mut mi = 0;
        let mut score = 0;
        for ox in oxs.chars() {
            if ox == 'o' {
                s[ni].1[mi] = true;
                score += a[mi].0;
            }
            mi += 1;
        }
        score += (ni + 1) as u32;
        s[ni].0 = score;

        max_score = if max_score > score {max_score} else {score};

        dprintln!("{} {} {}", mi, score, max_score);
    }

    dprintln!("{:?}", s);

    a.sort_by(|l, r| r.0.cmp(&l.0));

    dprintln!("{:?}", a);

    //

    for ni in 0..n {
        let mut ans = 0;
        let mut add_score = 0;
        for mi in 0..m {
            dprintln!("{} {} {}", s[ni].0, add_score, max_score);
            if s[ni].0 + add_score >= max_score {
                break;
            }

            if !s[ni].1[a[mi].1] {
                add_score += a[mi].0;
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }

}
