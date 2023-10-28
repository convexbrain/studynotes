use std::collections::*;

struct PermIter
{
    n: usize,
    k: usize,
    n_p_k: Vec<usize>,
    free: BTreeSet<usize>,
    first: bool,
    end: bool,
}

impl PermIter
{
    fn new(n: usize, k: usize) -> Self {
        assert!(n >= k);
        let n_p_k = (0..k).collect();
        let free = (k..n).collect();
        PermIter {
            n, k, n_p_k, free, first: true, end: false
        }
    }

    fn next(&mut self) -> Option<&[usize]> {
        if self.first {
            self.first = false;
        }
        else if self.end {
            return None;
        }
        else {
            for pos in (0..self.k).rev() {
                let c = self.n_p_k[pos];
                self.free.insert(c);
                if let Some(&nc) = self.free.range((c + 1)..).next() {
                    self.n_p_k[pos] = nc;
                    self.free.remove(&nc);
                    for i in (pos + 1)..self.k {
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
    
        Some(&self.n_p_k)
    }
}

#[test]
fn test_perm_iter() {
    let mut c = PermIter::new(4, 3);
    assert_eq!(c.next(), Some(vec![0, 1, 2].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 1, 3].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 2, 1].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 2, 3].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 3, 1].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 3, 2].as_ref()));
    assert_eq!(c.next(), Some(vec![1, 0, 2].as_ref()));
    assert_eq!(c.next(), Some(vec![1, 0, 3].as_ref()));
    assert_eq!(c.next(), Some(vec![1, 2, 0].as_ref()));
    assert_eq!(c.next(), Some(vec![1, 2, 3].as_ref()));
    assert_eq!(c.next(), Some(vec![1, 3, 0].as_ref()));
    assert_eq!(c.next(), Some(vec![1, 3, 2].as_ref()));
    assert_eq!(c.next(), Some(vec![2, 0, 1].as_ref()));
    assert_eq!(c.next(), Some(vec![2, 0, 3].as_ref()));
    assert_eq!(c.next(), Some(vec![2, 1, 0].as_ref()));
    assert_eq!(c.next(), Some(vec![2, 1, 3].as_ref()));
    assert_eq!(c.next(), Some(vec![2, 3, 0].as_ref()));
    assert_eq!(c.next(), Some(vec![2, 3, 1].as_ref()));
    assert_eq!(c.next(), Some(vec![3, 0, 1].as_ref()));
    assert_eq!(c.next(), Some(vec![3, 0, 2].as_ref()));
    assert_eq!(c.next(), Some(vec![3, 1, 0].as_ref()));
    assert_eq!(c.next(), Some(vec![3, 1, 2].as_ref()));
    assert_eq!(c.next(), Some(vec![3, 2, 0].as_ref()));
    assert_eq!(c.next(), Some(vec![3, 2, 1].as_ref()));
    assert_eq!(c.next(), None);
    assert_eq!(c.next(), None);
}
