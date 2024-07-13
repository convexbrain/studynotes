use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::{*, Bound::*}, cmp::*,
    rc::*, cell::*,
};

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

struct Tokens<'a>(std::str::SplitWhitespace<'a>);

#[allow(dead_code)]
impl<'a> Tokens<'a> {
    fn new(buf: &'a mut String) -> Self {
        std::io::stdin().read_to_string(buf).unwrap();
        Tokens(buf.split_whitespace())
    }
    fn release(self) -> std::str::SplitWhitespace<'a> {
        self.0
    }
    fn next_string(&mut self) -> String {
        self.0.next().unwrap().to_string()
    }
    fn next_bytes(&mut self) -> Vec<u8> {
        self.0.next().unwrap().as_bytes().to_vec()
    }
    fn next<T>(&mut self) -> T
    where T: std::str::FromStr, T::Err: std::fmt::Debug {
        self.0.next().unwrap().parse().unwrap()
    }
    fn collect<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<T> {
        (0..n).map(|_| self.next()).collect()
    }
    fn collect_index<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<(usize, T)> {
        (0..n).map(|i| (i, self.next())).collect()
    }
}

//#############################################################################

fn sub(vis: &mut BTreeMap<(u16, usize), usize>, a: u16, t: u16, n: usize, k: usize, cnt: usize, min_cnt: &mut usize) -> usize {
    let mut cc = usize::MAX;

    if vis.contains_key(&(a, k)) {
        cc = vis[&(a, k)];

        if cc < cnt {
            return cc;
        }
    }

    //debug!(a, k);

    if k == n && a == t {
        debug!(cnt);
        *min_cnt = cnt.min(*min_cnt);
        vis.insert((a, k), cnt);
        return cnt;
    }
    else if cnt < *min_cnt {
        for i in 0..(n + 1) {
            if i != k && i != k + 1 && i + 1 != k {
                let b = (a >> i) & 0x3;
                let m = 0x3 << i;
                let aa = (a & !m) | (b << k);
                let c = sub(vis, aa, t, n, i, cnt + 1, min_cnt);
                cc = c.min(cc);
                vis.insert((a, k), cc);
            }
        }
    }

    vis.insert((a, k), cc);
    return vis[&(a, k)];
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let s = tokens.next_bytes(); // Vec<u8>
    let t = tokens.next_bytes(); // Vec<u8>

    let s = s.iter().enumerate().fold(0_u16, |acc, x| acc + ((if *x.1 == b'W' {1} else {0}) << x.0));
    let t = t.iter().enumerate().fold(0_u16, |acc, x| acc + ((if *x.1 == b'W' {1} else {0}) << x.0));
    debug!(s, t);

    let mut vis = vec![vec![usize::MAX; 16]; 65536];
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), s, n));
    while let Some((cnt, a, k)) = que.pop() {
        if vis[a as usize][k] > cnt.0 {
            vis[a as usize][k] = cnt.0;

            if cnt.0 < vis[t as usize][n] {
                for i in 0..(n + 1) {
                    if i != k && i != k + 1 && i + 1 != k {
                        let b = (a >> i) & 0x3;
                        let m = 0x3 << i;
                        let aa = (a & !m) | (b << k);

                        que.push((Reverse(cnt.0 + 1), aa, i));
                    }
                }
            }
        }
    }

    if vis[t as usize][n] == usize::MAX {
        println!("-1");
    }
    else {
        println!("{}", vis[t as usize][n]);
    }
}
