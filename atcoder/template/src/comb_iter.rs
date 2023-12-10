//#############################################################################

#[derive(Debug, Clone)]
struct CombIter
{
    n: usize,
    k: usize,
}

#[derive(Debug, Clone)]
struct IterComb<'a>
{
    comb: &'a CombIter,
    n_c_k: Vec<usize>,
    first: bool,
    end: bool,
}

impl CombIter
{
    fn new(n: usize, k: usize) -> Self {
        assert!(n >= k);
        CombIter {n, k}
    }

    fn iter(&self) -> IterComb {
        let n_c_k = (0..self.k).collect();
        IterComb {
            comb: &self,
            n_c_k,
            first: true, end: false
        }
    }
}

impl<'a> Iterator for IterComb<'a>
{
    type Item = &'a[usize];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            if self.comb.k == 0 {
                self.end = true;
            }
        }
        else if self.end {
            return None;
        }
        else {
            let n = self.comb.n;
            let k = self.comb.k;

            for pos in (0..k).rev() {
                let c = self.n_c_k[pos] + 1;
                if c < n - k + 1 + pos {
                    for i in pos..k {
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

//#############################################################################

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

    for (i, c) in CombIter::new(5, 3).iter().enumerate() {
        assert_eq!(c, e[i]);
    }
}
