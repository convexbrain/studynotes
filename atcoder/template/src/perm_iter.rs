use std::collections::*;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct PermIter
{
    n: usize,
    k: usize,
}

#[derive(Debug, Clone)]
struct IterPerm<'a>
{
    perm: &'a PermIter,
    n_p_k: Vec<usize>,
    free: BTreeSet<usize>,
    first: bool,
    end: bool,
}

impl PermIter
{
    fn new(n: usize, k: usize) -> Self {
        assert!(n >= k);
        PermIter {n, k}
    }

    fn iter(&self) -> IterPerm {
        let n_p_k = (0..self.k).collect();
        let free = (self.k..self.n).collect();
        IterPerm {
            perm: &self,
            n_p_k, free,
            first: true, end: false,
        }
    }
}

impl<'a> Iterator for IterPerm<'a>
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
            let k = self.perm.k;

            for pos in (0..k).rev() {
                let c = self.n_p_k[pos];
                self.free.insert(c);
                if let Some(&nc) = self.free.range((c + 1)..).next() {
                    self.n_p_k[pos] = nc;
                    self.free.remove(&nc);
                    for i in (pos + 1)..k {
                        self.n_p_k[i] = self.free.pop_first().unwrap();
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
                std::mem::transmute(self.n_p_k.as_slice())
            }
        )
    }
}

///////////////////////////////////////////////////////////////////////////////

#[test]
fn test_perm_iter() {
    let e = [
        [0, 1, 2],
        [0, 1, 3],
        [0, 2, 1],
        [0, 2, 3],
        [0, 3, 1],
        [0, 3, 2],
        [1, 0, 2],
        [1, 0, 3],
        [1, 2, 0],
        [1, 2, 3],
        [1, 3, 0],
        [1, 3, 2],
        [2, 0, 1],
        [2, 0, 3],
        [2, 1, 0],
        [2, 1, 3],
        [2, 3, 0],
        [2, 3, 1],
        [3, 0, 1],
        [3, 0, 2],
        [3, 1, 0],
        [3, 1, 2],
        [3, 2, 0],
        [3, 2, 1],
    ];

    for (i, p) in PermIter::new(4, 3).iter().enumerate() {
        assert_eq!(p, e[i]);
    }
}
