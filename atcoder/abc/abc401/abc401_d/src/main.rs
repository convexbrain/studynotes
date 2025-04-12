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

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let n: usize = tokens.next();
    let k: usize = tokens.next();
    let mut s: Vec<char> = tokens.next_string().chars().collect();

    for i in 0..n {
        match s[i] {
            'o' => {
                if i > 0 {s[i - 1] = '.';}
                if i < n - 1 {s[i + 1] = '.';}
            },
            _ => {}
        }
    }

    let mut cnt_o = 0;
    let mut cnt_e = 0;
    let mut cnt_q = 0;
    let mut cont = 0;
    let mut max_o = 0;
    for i in 0..n {
        match s[i] {
            'o' => {
                cnt_o += 1;
                if cont == 1 {
                    s[i - 1] = '!';
                    cnt_e += 1;
                }
                else if cont > 1 {
                    max_o += (cont + 1) / 2;
                    cnt_q += 1;
                }
                cont = 0;
            },
            '?' => {
                cont += 1;
            },
            _ => {
                if cont == 1 {
                    s[i - 1] = '!';
                    cnt_e += 1;
                }
                else if cont > 1 {
                    max_o += (cont + 1) / 2;
                    cnt_q += 1;
                }
                cont = 0;
            }
        }
    }
    if cont == 1 {
        s[n - 1] = '!';
        cnt_e += 1;
    }
    else if cont > 1 {
        max_o += (cont + 1) / 2;
        cnt_q += 1;
    }

    for i in 0..n {
        match s[i] {
            '!' => {
                if cnt_e == k - cnt_o - max_o {
                    print!("o");
                }
                else if cnt_q == 0 && 0 == k - cnt_o {
                    print!(".");
                }
                else {
                    print!("?");
                }
            },
            _ => {
                print!("{}", s[i]);
            }
        }
    }
    println!();
}
