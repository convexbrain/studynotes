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
    fn new(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_to_string(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
    }
    fn new_line(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_line(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
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
}

//#############################################################################

// (g, x, y) s.t. a x + b y = gcd(a, b) = g
fn ext_euclid<N>(a: N, b: N) -> (N, N, N)
where N: Default + Div<Output=N> + Mul<Output=N> + Sub<Output=N> + Eq + Copy + std::fmt::Debug
{
    let zero = N::default();
    let one = b / b;

    let mut r0 = a;
    let mut s0 = one;
    let mut t0 = zero;
    let mut r1 = b;
    let mut s1 = zero;
    let mut t1 = one;

    loop {
        let q1 = r0 / r1;
        let r2 = r0 - q1 * r1;
        let s2 = s0 - q1 * s1;
        let t2 = t0 - q1 * t1;

        if r2 == zero {
            return (r1, s1, t1);
        }

        (r0, r1) = (r1, r2);
        (s0, s1) = (s1, s2);
        (t0, t1) = (t1, t2);
    }
}

fn mod_div<N>(x: N, y: N, m: N) -> N
where N: Default + Div<Output=N> + Mul<Output=N> + Sub<Output=N> + Ord + Copy + Rem<Output=N> + Add<Output=N> + std::fmt::Debug
{
    let zero = N::default();
    let one = m / m;

    let (gcd, y_inv, _) = ext_euclid(y, m);

    if gcd != one {panic!();}

    let y_inv = if y_inv < zero {y_inv + m} else {y_inv};

    (x * y_inv) % m
}

//#############################################################################

fn comb(x: i64, y: i64, p: i64) -> (i64, i64, i64) {
    assert!(x >= y);

    let mut nom = 1;
    let mut den = 1;
    let mut nn = x;
    for dd in 1..=y {
        nom = (nom * nn) % p;
        den = (den * dd) % p;

        nn -= 1;
    }
    (mod_div(nom, den, p), x, nn)
}

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let na: i64 = tokens.next();
    let no: i64 = tokens.next();
    let nb: i64 = tokens.next();
    let ng: i64 = tokens.next();
    let p = 998244353;

    let (mut com_ao, mut head_ao, mut tail_ao) = comb(na + no, no, p);
    let (mut com_bg, mut head_bg, mut tail_bg) = comb(nb + ng - 1, ng - 1, p);

    let mut cnt = 0;

    for b in 0..=nb {
        debug!(com_ao, com_bg);
        let c = (com_ao * com_bg) % p;
        cnt = (cnt + c) % p;

        if b < nb {
            head_ao += 1;
            tail_ao += 1;
            com_ao = (com_ao * head_ao) % p;
            com_ao = mod_div(com_ao, tail_ao, p);

            com_bg = (com_bg * tail_bg) % p;
            com_bg = mod_div(com_bg, head_bg, p);
            head_bg -= 1;
            tail_bg -= 1;
        }
    }

    println!("{cnt}");
}
