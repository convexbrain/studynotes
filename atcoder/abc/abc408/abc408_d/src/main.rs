use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::{*, Bound::*}, cmp::*,
    rc::*, cell::*,
};

#[allow(dead_code)]
trait Int: std::fmt::Debug + Copy + Default + Ord + Eq + ShrAssign + SubAssign +
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> + BitAnd<Output=Self> +
    TryInto<usize> + TryFrom<usize>
{ fn chk_mul(self, rhs: Self) -> Option<Self>; }
impl Int for u8 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u16 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u32 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u64 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u128 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for usize { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i8 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i16 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i32 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i64 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i128 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for isize { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }

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

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let t: usize = tokens.next();
    for _ in 0..t {
        let n: usize = tokens.next();
        let s = tokens.next_string(); // String
        let mut v = VecDeque::new();
        let mut len0 = 0_usize;
        let mut len1 = 0_usize;
        let mut cnt = 0;
        for (i, c) in s.char_indices() {
            if c == '1' {
                len1 += 1;
                if len0 > 0 {
                    v.push_back((0_u8, len0));
                    cnt += len0;
                }
                len0 = 0;
            }
            else {
                len0 += 1;
                if len1 > 0 {
                    v.push_back((1, len1));
                }
                len1 = 0;
            }
        }
        if len0 > 0 {
            v.push_back((0, len0));
                    cnt += len0;
        }
        if len1 > 0 {
            v.push_back((1, len1));
        }

        if let Some(vi) = v.front() {
            if vi.0 == 0 {
                cnt -= vi.1;
                v.pop_front();
            }
        }
        if let Some(vi) = v.back() {
            if vi.0 == 0 {
                cnt -= vi.1;
                v.pop_front();
            }
        }
        debug!(v, cnt);

        while v.len() >= 3 {
            debug!(v, cnt);
            let front_dec = if v[0].1 <= v[1].1 {
                Some(v[1].1 - v[0].1)
            }
            else {
                None
            };
            let back_dec = if v[v.len() - 2].1 >= v[v.len() - 1].1 {
                Some(v[v.len() - 2].1 - v[v.len() - 1].1)
            }
            else {
                None
            };
            if front_dec.is_some() && back_dec.is_some() {
                if front_dec.unwrap() > back_dec.unwrap() {
                    cnt -= front_dec.unwrap();
                    v.pop_front();
                    v.pop_front();
                }
                else {
                    cnt -= back_dec.unwrap();
                    v.pop_back();
                    v.pop_back();
                }
            }
            else {
                if front_dec.is_some() {
                    cnt -= front_dec.unwrap();
                    v.pop_front();
                    v.pop_front();
                }
                else if back_dec.is_some() {
                    cnt -= back_dec.unwrap();
                    v.pop_back();
                    v.pop_back();
                }
            }
        }

        println!("{cnt}");
    }
}
