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

fn sub(memo: &mut HashMap<u64, (u64, u64)>, n: u64) -> (u64, u64) {
    if memo.contains_key(&n) {
        memo[&n]
    }
    else {
        let mut p = 0;
        let mut q = n;
        while q & 1 == 0 && q > 0 {
            q >>= 1;
            p += 1;
        }
        memo.insert(n, (p, q));
        (p, q)
    }
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let l: u64 = tokens.next();
    let r: u64 = tokens.next();

    let mut memo = HashMap::new();

    let mut rs = Vec::new();
    {
        let (mut rp, mut rq) = sub(&mut memo, r);
        rs.push((rp, rq));
        while rq > 1 {
            let (nrp, nrq) = sub(&mut memo, rq - 1);
            (rp, rq) = (rp + nrp, nrq);
            rs.push((rp, rq));
        }
    }
    debug!(rs);

    let mut ls = Vec::new();
    {
        if l > 0 {
            let (mut lp, mut lq) = sub(&mut memo, l);
            ls.push((lp, lq));
            while lq > 1 {
                let (nlp, nlq) = sub(&mut memo, lq + 1);
                (lp, lq) = (lp + nlp, nlq);
                ls.push((lp, lq));
            }
        }
        else {
            let (p, _) = rs.last().unwrap();
            ls.push((*p, 0));
        }
    }
    debug!(ls);

    let ms = if l > 0 {
        let (llp, _) = ls.last().unwrap();
        let (lrp, _) = rs.last().unwrap();
        lrp - llp - 1
    }
    else {
        0
    };

    let m = ls.len() + ms as usize + rs.len() - 1;
    println!("{m}");
    let mut al = l;
    for (lp, lq) in ls.iter().skip(1) {
        let ar = (1 << lp) * lq;
        println!("{al} {ar}");
        al = ar;
    }
    debug!(al);
    for _ in 0..ms {
        let ar = al * 2;
        println!("{al} {ar}");
        al = ar;
    }
    debug!(al);
    for (rp, rq) in rs.iter().rev() {
        let ar = (1 << rp) * rq;
        println!("{al} {ar}");
        al = ar;
    }
}
