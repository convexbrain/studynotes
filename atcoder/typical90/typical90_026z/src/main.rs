use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;
use std::cmp::*;

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ( $($x:tt)* ) => {};
}

#[cfg(debug_assertions)]
macro_rules! debug {
    () => {
        eprintln!("[@{}]", line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            ref tmp => {
                eprintln!("[@{}] {} = {:?}",
                    line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(debug!($val)),+,)
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();

    debug!(n);

    let mut tree = vec![vec![]; n];
    let mut grp = vec![false; n];

    for _ in 0..(n - 1) {
        let a: usize = token.next().unwrap().parse().unwrap();
        let b: usize = token.next().unwrap().parse().unwrap();
    
        debug!(a, b);

        let a = a - 1;
        let b = b - 1;

        tree[a].push(b);
        tree[b].push(a);
    }

    debug!(tree);

    let mut que = VecDeque::new();
    let mut vis = HashSet::new();

    que.push_back((0_usize, true));
    while let Some((no, gr)) = que.pop_front() {
        grp[no] = gr;

        vis.insert(no);
        for &ch in tree[no].iter() {
            if !vis.contains(&ch) {
                que.push_back((ch, !gr));
            }
        }

    }

    debug!(grp);

    let true_num = grp.iter().fold(0, |acc, &x| acc + if x {1} else {0});
    let sel = true_num > n / 2;

    let mut cnt = 0;
    for (i, &val) in grp.iter().enumerate() {
        if val == sel {
            print!("{} ", i + 1);
            cnt += 1;
            if cnt >= n / 2 {
                break;
            }
        }
    }
    
    println!("");
}
