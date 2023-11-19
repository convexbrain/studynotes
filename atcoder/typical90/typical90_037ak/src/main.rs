use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::{*, Bound::*}, cmp::*,
    str, rc::*, cell::*,
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

#[derive(Clone)]
struct STree<T, F, S>
{
    n: usize,
    nleafs: usize,
    vec: Vec<Option<T>>,
    lazy: Vec<Option<T>>,
    assoc_op: F,
    assoc_scl: S,
}

impl<T: std::fmt::Debug, F, S> std::fmt::Debug for STree<T, F, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("n:{}, nleafs:{}", self.n, self.nleafs))?;

        f.write_str("\nvec:[")?;
        for (n, v) in self.vec.iter().enumerate() {
            if Self::h_depth_of(n) > 0 {
                if Self::h_breadth_of(n) == 0 {
                    f.write_str("; ")?;
                }
                else {
                    f.write_str(", ")?;
                }
            }
            f.write_fmt(format_args!("{:?}", v))?;
        }
        f.write_str("]")?;

        f.write_str("\nlazy:[")?;
        for (n, v) in self.lazy.iter().enumerate() {
            if Self::h_depth_of(n) > 0 {
                if Self::h_breadth_of(n) == 0 {
                    f.write_str("; ")?;
                }
                else {
                    f.write_str(", ")?;
                }
            }
            f.write_fmt(format_args!("{:?}", v))?;
        }
        f.write_str("]")
    }
}

impl<T, F, S> STree<T, F, S>
{
    fn h_depth_of(node: usize) -> usize {
        (node + 1).ilog2() as usize
    }
    
    fn h_breadth_of(node: usize) -> usize {
        let depth = Self::h_depth_of(node);
        node - ((1 << depth) - 1)
    }

    fn h_range_of(node: usize, nleafs: usize) -> (usize, usize) {
        let depth = Self::h_depth_of(node);
        let breadth = Self::h_breadth_of(node);

        let width = nleafs / (1 << depth);
        let si = breadth * width;
        let ee = si + width;

        (si, ee)
    }

    fn h_left_of(node: usize) -> usize {
        let depth = Self::h_depth_of(node);
        let breadth = Self::h_breadth_of(node);

        ((1 << (depth + 1)) - 1) + (breadth << 1)
    }
    
    fn h_right_of(node: usize) -> usize {
        Self::h_left_of(node) + 1
    }

    fn h_parent_of(node: usize) -> usize {
        let depth = Self::h_depth_of(node);
        let breadth = Self::h_breadth_of(node);

        ((1 << (depth - 1)) - 1) + (breadth >> 1)
    }
    
    fn h_sanitize<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
        let si = match range.start_bound() {
            Included(&b) => {b},
            Excluded(&b) => {b + 1},
            Unbounded => {0},
        };
        let ee = match range.end_bound() {
            Included(&b) => {b + 1},
            Excluded(&b) => {b},
            Unbounded => {n},
        };

        assert!(si < ee);
        assert!(ee <= n);

        (si, ee)
    }
}
    
impl<T: Copy, F, S> STree<T, F, S>
where F: Fn(T, T) -> T, S: Fn(T, usize) -> T
{
    fn new(n: usize, assoc_op: F, assoc_scl: S) -> Self {
        let (nnodes, nleafs) = if n == 0 {
            (0, 0)
        }
        else if n == 1 {
            (1, 1)
        }
        else {
            let log_n_ceil = (n - 1).ilog2() + 1;
            let nleafs = 1 << log_n_ceil;
            (nleafs * 2 - 1, nleafs)
        };


        STree {n, nleafs, vec: vec![None; nnodes], lazy: vec![None; nnodes], assoc_op, assoc_scl}
    }

    fn assoc_op_opt(&self, oa: Option<T>, ob: Option<T>) -> Option<T> {
        if let Some(a) = oa {
            if let Some(b) = ob {
                Some( (self.assoc_op)(a, b) )
            }
            else {
                oa
            }
        }
        else {
            ob
        }
    }

    fn update<R: RangeBounds<usize>>(&mut self, range: R, value: T) {
        let (si, ee) = Self::h_sanitize(range, self.n);

        let mut que = VecDeque::new();
        que.push_back((0, (si, ee), value));

        while let Some((node, trange, val)) = que.pop_front() {
            let nrange = Self::h_range_of(node, self.nleafs);

            let val_r = (self.assoc_scl)(val, trange.1 - trange.0);
            self.vec[node] = self.assoc_op_opt(self.vec[node], Some(val_r));

            if nrange == trange {
                self.lazy[node] = self.assoc_op_opt(self.lazy[node], Some(val));
            }
            else {
                let half = (nrange.0 + nrange.1) / 2;

                if let Some(lazy) = self.lazy[node].take() {
                    que.push_front((Self::h_left_of(node), (nrange.0, half), lazy));
                    que.push_front((Self::h_right_of(node), (half, nrange.1), lazy));
                }

                if trange.1 <= half {
                    que.push_back((Self::h_left_of(node), trange, val));
                }
                else if trange.0 >= half {
                    que.push_back((Self::h_right_of(node), trange, val));
                }
                else {
                    que.push_back((Self::h_left_of(node), (trange.0, half), val));
                    que.push_back((Self::h_right_of(node), (half, trange.1), val));
                }
            }
        }
    }

    fn eval<R: RangeBounds<usize>>(&mut self, range: R) -> Option<T> {
        let (si, ee) = Self::h_sanitize(range, self.n);

        let mut value = None;

        let mut que = VecDeque::new();
        que.push_back((0, Ok((si, ee))));

        while let Some((node, typ)) = que.pop_front() {
            let nrange = Self::h_range_of(node, self.nleafs);

            match typ {
                Ok(trange) => {
                    if nrange == trange {
                        value = self.assoc_op_opt(value, self.vec[node]);
                    }
                    else {
                        let half = (nrange.0 + nrange.1) / 2;
        
                        if let Some(lazy) = self.lazy[node].take() {
                            que.push_front((Self::h_left_of(node), Err(lazy)));
                            que.push_front((Self::h_right_of(node), Err(lazy)));
                        }
        
                        if trange.1 <= half {
                            que.push_back((Self::h_left_of(node), Ok(trange)));
                        }
                        else if trange.0 >= half {
                            que.push_back((Self::h_right_of(node), Ok(trange)));
                        }
                        else {
                            que.push_back((Self::h_left_of(node), Ok((trange.0, half))));
                            que.push_back((Self::h_right_of(node), Ok((half, trange.1))));
                        }
                    }
                },
                Err(lazy) => {
                    let val_r = (self.assoc_scl)(lazy, nrange.1 - nrange.0);
                    self.vec[node] = self.assoc_op_opt(self.vec[node], Some(val_r));

                    self.lazy[node] = self.assoc_op_opt(self.lazy[node], Some(lazy));
                }
            }
        }

        value
    }
}

struct IterSTree<'a, T, F, S> {
    stree: &'a mut STree<T, F, S>,
    idx: usize,
}

impl<'a, T: Copy, F, S> Iterator for IterSTree<'a, T, F, S>
where F: Fn(T, T) -> T, S: Fn(T, usize) -> T
{
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.stree.n {
            let next = self.stree.eval(self.idx..=self.idx);
            self.idx += 1;
            Some(next)
        }
        else {
            None
        }
    }
}

impl<T: Copy, F, S> STree<T, F, S>
{
    fn iter(&mut self) -> IterSTree<'_, T, F, S> {
        IterSTree { stree: self, idx: 0 }
    }
}

//#############################################################################

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let w: i16 = token.next().unwrap().parse().unwrap();
    let n: usize = token.next().unwrap().parse().unwrap();

    let lrv: Vec<(i16, i16, u32)> = (0..n).map(|_|
        (
            token.next().unwrap().parse().unwrap(),
            token.next().unwrap().parse().unwrap(),
            token.next().unwrap().parse().unwrap(),
        )
    ).collect();

    debug!(lrv);

    let mut val0 = STree::new(w as usize + 1,
        |a: u64, b| a.max(b),
        |b, _l| b
    );
    val0.update(0..=0, 0);

    for (l, r, v) in lrv.iter() {
        let mut val1 = val0.clone();

        for wi in 1..=w {
            let si = wi - r;
            let ei = wi - l;

            if ei >= 0 {
                let si = si.max(0) as usize;
                let ei = ei.max(0) as usize;

                if let Some(m) = val0.eval(si..=ei) {
                    let wi = wi as usize;
                    val1.update(wi..=wi, m + *v as u64);
                }
            }
        }

        val0 = val1;
        //debug!(val0);
    }

    let w = w as usize;
    if let Some(ans) = val0.eval(w..=w) {
        println!("{ans}");
    }
    else {
        println!("-1");
    }
}
