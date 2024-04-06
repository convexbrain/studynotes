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

#[derive(Debug, Clone)]
struct Vec2D<T> {
    vec_row_major: Vec<T>,
    nrows: usize,
    ncols: usize,
}

impl<T> Vec2D<T> {
    fn from(vec_row_major: Vec<T>, (nrows, ncols): (usize, usize)) -> Self {
        assert!(vec_row_major.len() >= nrows * ncols);
        Self {vec_row_major, nrows, ncols}
    }

    fn release(self) -> Vec<T> {
        self.vec_row_major
    }
}

impl<T> Vec2D<T> {
    fn get(&self, row: isize, col: isize) -> Option<&T> {
        if row < 0 || col < 0 || !(row < self.nrows as isize) || !(col < self.ncols as isize) {
            None
        }
        else {
            Some(&self.vec_row_major[row as usize * self.ncols + col as usize])
        }
    }

    fn get_mut(&mut self, row: isize, col: isize) -> Option<&mut T> {
        if row < 0 || col < 0 || !(row < self.nrows as isize) || !(col < self.ncols as isize) {
            None
        }
        else {
            Some(&mut self.vec_row_major[row as usize * self.ncols + col as usize])
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.nrows);
        assert!(index.1 < self.ncols);
        &self.vec_row_major[index.0 * self.ncols + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.nrows);
        assert!(index.1 < self.ncols);
        &mut self.vec_row_major[index.0 * self.ncols + index.1]
    }
}

//#############################################################################

fn dsub(m: &Vec2D<char>, nodes: &[(usize, usize)], from: usize, d: &mut[u32]) {
    let h = m.nrows;
    let w = m.ncols;

    let mut dist = Vec2D::from(vec![u32::MAX; h * w], (h, w));

    let mut que = BinaryHeap::new();
    que.push((Reverse(0), nodes[from]));
    while let Some(q) = que.pop() {
        if dist[q.1] > q.0.0 {
            dist[q.1] = q.0.0;

            for dir in 0..4 {
                let r = q.1.0 as isize;
                let c = q.1.1 as isize;
                let r = r + if dir == 0 {-1} else if dir == 1 {1} else {0};
                let c = c + if dir == 2 {-1} else if dir == 3 {1} else {0};
                if let Some(x) = m.get(r, c) {
                    if *x != '#' {
                        que.push((Reverse(q.0.0 + 1), (r as usize, c as usize)));
                    }
                }
            }
        }
    }

    for i in 0..nodes.len() {
        d[i] = dist[nodes[i]];
    }
}

fn ssub(n: usize, d2: &Vec<Vec<u32>>, ene: &[u32], from: usize, e: u32, vis: &mut BTreeSet<usize>) -> bool {
    debug!(from, e);
    if from == n {
        return true;
    }
    else if !vis.contains(&from) {
        vis.insert(from);

        for to in 0..(n + 1) {
            if to != from {
                let ce = d2[from][to];
                debug!(to, ce);
                if e >= ce {
                    if to == n {
                        return true;
                    }
                    else if ssub(n, d2, ene, to, ene[to], vis) {
                        return true;
                    }
                }
            }
        }

        vis.remove(&from);
    }

    false
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let h: usize = tokens.next();
    let w: usize = tokens.next();

    let mut m = Vec2D::from(vec!['.'; h * w], (h, w));
    let mut spos = (0, 0);
    let mut tpos = (0, 0);

    for hi in 0..h {
        let a = tokens.next_string(); // String

        for (wi, ai) in a.chars().enumerate() {
            m[(hi, wi)] = ai;

            if ai == 'S' {
                spos = (hi, wi);
            }
            else if ai == 'T' {
                tpos = (hi, wi);
            }
        }
    }
    debug!(m);

    let n: usize = tokens.next();
    let mut rc = Vec::new();
    let mut ene = Vec::new();
    let mut snode = None;
    for i in 0..n {
        let r: usize = tokens.next();
        let c: usize = tokens.next();
        let e: u32 = tokens.next();

        let r = r - 1;
        let c = c - 1;

        rc.push((r, c));
        ene.push(e);
        if spos == (r, c) {
            snode = Some(i);
        }
    }
    debug!(rc, ene, snode);

    //
    
    if snode.is_none() {
        println!("No");
        return;
    }
    let snode = snode.unwrap();

    //

    let mut nodes = vec![(0, 0); n + 1];
    nodes[n] = tpos;
    for (i, rci) in rc.iter().enumerate() {
        nodes[i] = (rci.0, rci.1);
    }
    debug!(nodes);

    let mut d2 = vec![vec![u32::MAX; n + 1]; n + 1];
    for from in 0..(n + 1) {
        dsub(&m, &nodes, from, &mut d2[from]);
    }
    debug!(d2);

    //

    let mut que = VecDeque::new();
    let mut vis = BTreeSet::new();

    que.push_front(snode);
    while let Some(node) = que.pop_front() {
        if !vis.contains(&node) {
            vis.insert(node);

            for nnode in 0..(n + 1) {
                if node != nnode && d2[node][nnode] <= ene[node] {
                    if nnode == n {
                        println!("Yes");
                        return;
                    }
                    else {
                        que.push_front(nnode);
                    }
                }
            }
        }
    }
    println!("No");
}
