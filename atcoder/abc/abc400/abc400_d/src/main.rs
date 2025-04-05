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

fn sub(map: &mut Vec<Vec<i8>>, h: usize, w: usize, i: usize, j: usize, c: usize, d: usize, kick: usize, min_kick: &mut usize) {
    //debug!(i, j);
    if map[i][j] == 0 {
        if (i, j) == (c, d) {
            *min_kick = kick.min(*min_kick);
        }
        else {
            map[i][j] = 1;
            if i > 0 {
                sub(map, h, w, i - 1, j, c, d, kick, min_kick);
            }
            if i < h - 1 {
                sub(map, h, w, i + 1, j, c, d, kick, min_kick);
            }
            if j > 0 {
                sub(map, h, w, i, j - 1, c, d, kick, min_kick);
            }
            if j < w - 1 {
                sub(map, h, w, i, j + 1, c, d, kick, min_kick);
            }
            map[i][j] = 0;
            if kick < *min_kick {
                let mut f1 = false;
                let mut f2 = false;
                if i > 0 && map[i - 1][j] == -1 { f1 = true; map[i - 1][j] = 0; }
                if i > 1 && map[i - 2][j] == -1 { f2 = true; map[i - 2][j] = 0; }
                if f1 || f2 { sub(map, h, w, i, j, c, d, kick + 1, min_kick); }
                if f1 { map[i - 1][j] = -1; }
                if f2 { map[i - 2][j] = -1; }

                let mut f1 = false;
                let mut f2 = false;
                if i < h - 1 && map[i + 1][j] == -1 { f1 = true; map[i + 1][j] = 0; }
                if i < h - 2 && map[i + 2][j] == -1 { f2 = true; map[i + 2][j] = 0; }
                if f1 || f2 { sub(map, h, w, i, j, c, d, kick + 1, min_kick); }
                if f1 { map[i + 1][j] = -1; }
                if f2 { map[i + 2][j] = -1; }
            
                let mut f1 = false;
                let mut f2 = false;
                if j > 0 && map[i][j - 1] == -1 { f1 = true; map[i][j - 1] = 0; }
                if j > 1 && map[i][j - 2] == -1 { f2 = true; map[i][j - 2] = 0; }
                if f1 || f2 { sub(map, h, w, i, j, c, d, kick + 1, min_kick); }
                if f1 { map[i][j - 1] = -1; }
                if f2 { map[i][j - 2] = -1; }

                let mut f1 = false;
                let mut f2 = false;
                if j < w - 1 && map[i][j + 1] == -1 { f1 = true; map[i][j + 1] = 0; }
                if j < w - 2 && map[i][j + 2] == -1 { f2 = true; map[i][j + 2] = 0; }
                if f1 || f2 { sub(map, h, w, i, j, c, d, kick + 1, min_kick); }
                if f1 { map[i][j + 1] = -1; }
                if f2 { map[i][j + 2] = -1; }
            }
        }
    }
}

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let mut map = vec![vec![0; w]; h];
    for i in 0..h {
        let s = tokens.next_string(); // String
        for (j, c) in s.char_indices() {
            if c == '#' {
                map[i][j] = -1;
            }
        }
    }
    let a: usize = tokens.next();
    let b: usize = tokens.next();
    let c: usize = tokens.next();
    let d: usize = tokens.next();

    let mut ans = usize::MAX;
    sub(&mut map, h, w, a - 1, b - 1, c - 1, d - 1, 0, &mut ans);
    println!("{ans}");
}
