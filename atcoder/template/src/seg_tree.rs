use std::{ops::{*, Bound::*}, collections::*};

//#############################################################################

#[derive(Clone)]
struct SegTree<T, F, S>
{
    n: usize,
    nleafs: usize,
    vec: Vec<Option<T>>,
    assoc_op: F,
    assoc_scl: S,
}

impl<T: std::fmt::Debug, F, S> std::fmt::Debug for SegTree<T, F, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("n:{}, nleafs:{}, vec:{:?}", self.n, self.nleafs, self.vec))
    }
}

impl<T, F, S> SegTree<T, F, S>
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
    
impl<T: Copy, F, S> SegTree<T, F, S>
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


        SegTree {n, nleafs, vec: vec![None; nnodes], assoc_op, assoc_scl}
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
        que.push_back((0, Ok((si, ee))));

        while let Some((node, typ)) = que.pop_front() {
            let nrange = Self::h_range_of(node, self.nleafs);

            match typ {
                Ok(trange) => { // downward
                    if nrange == trange {
                        let value_r = (self.assoc_scl)(value, nrange.1 - nrange.0);

                        self.vec[node] = self.assoc_op_opt(self.vec[node], Some(value_r));

                        if node > 0 {
                            que.push_back((Self::h_parent_of(node), Err(value_r)));
                        }
        
                        // TODO: lazy evaluation
                    }
                    else {
                        let half = (nrange.0 + nrange.1) / 2;
        
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
                Err(value_r) => { // updaward
                    self.vec[node] = self.assoc_op_opt(self.vec[node], Some(value_r));

                    if node > 0 {
                        que.push_back((Self::h_parent_of(node), Err(value_r)));
                    }
                },
            }
        }
    }

    fn eval<R: RangeBounds<usize>>(&mut self, range: R) -> Option<T> {
        let (si, ee) = Self::h_sanitize(range, self.n);

        let mut value = None;

        let mut que = VecDeque::new();
        que.push_back((0, (si, ee)));

        while let Some((node, trange)) = que.pop_front() {
            let nrange = Self::h_range_of(node, self.nleafs);

            if nrange == trange {
                value = self.assoc_op_opt(value, self.vec[node]);
            }
            else {
                let half = (nrange.0 + nrange.1) / 2;

                // TODO: lazy evaluation

                if trange.1 <= half {
                    que.push_back((Self::h_left_of(node), trange));
                }
                else if trange.0 >= half {
                    que.push_back((Self::h_right_of(node), trange));
                }
                else {
                    que.push_back((Self::h_left_of(node), (trange.0, half)));
                    que.push_back((Self::h_right_of(node), (half, trange.1)));
                }
            }
        }

        value
    }
}

// TODO: iterator

//#############################################################################

#[test]
fn test_seg_tree() {
    let mut t = SegTree::new(6,
        |a, b| a + b,
        |b, l| b * l as i32
    );

    t.update(1..=4, 1);

    assert_eq!(t.eval(..), Some(4));
    assert_eq!(t.eval(1..=3), Some(3));
    //assert_eq!(t.eval(3..=4), Some(2)); // TODO: lazy evaluation
    //assert_eq!(t.eval(2..=2), Some(1)); // TODO: lazy evaluation
    assert_eq!(t.eval(0..=0), None);

    dbg!(t);
}
