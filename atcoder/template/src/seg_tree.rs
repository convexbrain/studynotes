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

impl<T: Copy, F, S> SegTree<T, F, S>
where F: Fn(T, T) -> T, S: Fn(T, usize) -> T
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
    
    fn h_sanitize<R: RangeBounds<usize>>(range: R, n: usize) -> Result<(usize, usize), ()> {
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

        if si >= ee || si >= n || ee > n {
            return Err(());
        }

        Ok((si, ee))
    }
    
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

    fn update<R: RangeBounds<usize>>(&mut self, range: R, value: T) -> Result<(), ()> {
        let (si, ee) = Self::h_sanitize(range, self.n)?;

        let mut que = VecDeque::new();
        que.push_back((0, Ok((si, ee))));

        while let Some((node, typ)) = que.pop_front() {
            let nrange = Self::h_range_of(node, self.nleafs);

            match typ {
                Ok(trange) => {
                    if nrange == trange {
                        let value_r = (self.assoc_scl)(value, nrange.1 - nrange.0);
        
                        if let Some(v) = self.vec[node] {
                            self.vec[node] = Some( (self.assoc_op)(v, value_r) );
                        }
                        else {
                            self.vec[node] = Some(value_r);
                        }

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
                Err(value_r) => {
                    if let Some(v) = self.vec[node] {
                        self.vec[node] = Some( (self.assoc_op)(v, value_r) );
                    }
                    else {
                        self.vec[node] = Some(value_r);
                    }

                    if node > 0 {
                        que.push_back((Self::h_parent_of(node), Err(value_r)));
                    }
                },
            }
        }

        Ok(())
    }

    fn eval<R: RangeBounds<usize>>(&mut self, range: R) -> Result<Option<T>, ()> {
        let (si, ee) = Self::h_sanitize(range, self.n)?;

        let mut value = None;

        let mut que = VecDeque::new();
        que.push_back((0, (si, ee)));

        while let Some((node, trange)) = que.pop_front() {
            let nrange = Self::h_range_of(node, self.nleafs);

            if nrange == trange {
                if let Some(some_value) = value {
                    if let Some(v) = self.vec[node] {
                        value = Some(
                            (self.assoc_op)(v, some_value)
                        );
                    }
                }
                else {
                    value = self.vec[node];
                }
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

        Ok(value)
    }
}

//#############################################################################

#[test]
fn test_seg_tree() {
    let mut t = SegTree::new(6,
        |a: i32, b| a + b,
        |b, l| b * l as i32
    );

    t.update(1..=4, 1).unwrap();

    assert_eq!(t.eval(..).unwrap(), Some(4));
    assert_eq!(t.eval(1..=3).unwrap(), Some(3));
    //assert_eq!(t.eval(3..=4).unwrap(), Some(2)); // TODO: lazy evaluation
    //assert_eq!(t.eval(2..=2).unwrap(), Some(1)); // TODO: lazy evaluation
    assert_eq!(t.eval(0..=0).unwrap(), None);

    dbg!(t);
}
