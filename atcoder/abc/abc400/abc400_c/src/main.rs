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

fn isqrt<N>(n: N, r0: N) -> N
where N: Default + Copy + Add<Output=N> + Div<Output=N> + Mul<Output=N> + PartialOrd<N>
{
    let zero = N::default();
    if n == zero {
        zero
    }
    else {
        let one = n / n;
        let two = one + one;
        let mut l = one;
        let mut r = r0;
        while l + one < r {
            let c = (l + r) / two;
            if c * c > n {
                r = c;
            }
            else {
                l = c;
            }
        }

        l
    }
}

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let n: u64 = tokens.next();

    let mut ans = 0;

    let mut x0 = n;
    for a in 1.. {
        x0 /= 2;

        if x0 == 0 {
            break;
        }

        let b = isqrt(x0, 1000000001);
        let b2 = (b + 1) / 2;
        debug!(a, b, b2);
        ans += b2;
        debug!(2_u64.pow(a) * b * b);
    }
    println!("{ans}");
}
