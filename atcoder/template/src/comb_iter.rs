///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct CombIter
{
    n: usize,
    k: usize,
    n_c_k: Vec<usize>,
    first: bool,
    end: bool,
}

impl CombIter
{
    fn new(n: usize, k: usize) -> Self {
        assert!(n >= k);
        let n_c_k = (0..k).collect();
        CombIter {
            n, k, n_c_k, first: true, end: false
        }
    }
}

impl<'a> Iterator for &'a mut CombIter
{
    type Item = &'a[usize];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
        }
        else if self.end {
            return None;
        }
        else {
            for pos in (0..self.k).rev() {
                let c = self.n_c_k[pos] + 1;
                if c < self.n - self.k + 1 + pos {
                    for i in pos..self.k {
                        self.n_c_k[i] = c + i - pos;
                    }
                    break;
                }
                else {
                    if pos == 0 {
                        self.end = true;
                        return None;
                    }
                }
            }
        }
    
        Some(
            unsafe {
                // self is borrowed as `&mut`, but this returns its contents as `&`.
                // It violates `&mut` constraints.
                std::mem::transmute(self.n_c_k.as_slice())
            }
        )
    }
}

///////////////////////////////////////////////////////////////////////////////

#[test]
fn test_comb_iter() {
    let e = [
        [0, 1, 2],
        [0, 1, 3],
        [0, 1, 4],
        [0, 2, 3],
        [0, 2, 4],
        [0, 3, 4],
        [1, 2, 3],
        [1, 2, 4],
        [1, 3, 4],
        [2, 3, 4],
    ];

    for (i, c) in CombIter::new(5, 3).into_iter().enumerate() {
        assert_eq!(c, e[i]);
    }
}
