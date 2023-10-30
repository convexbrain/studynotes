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

    fn next(&mut self) -> Option<&[usize]> {
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
    
        Some(&self.n_c_k)
    }
}

///////////////////////////////////////////////////////////////////////////////

#[test]
fn test_comb_iter() {
    let mut c = CombIter::new(5, 3);
    assert_eq!(c.next(), Some(vec![0, 1, 2].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 1, 3].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 1, 4].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 2, 3].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 2, 4].as_ref()));
    assert_eq!(c.next(), Some(vec![0, 3, 4].as_ref()));
    assert_eq!(c.next(), Some(vec![1, 2, 3].as_ref()));
    assert_eq!(c.next(), Some(vec![1, 2, 4].as_ref()));
    assert_eq!(c.next(), Some(vec![1, 3, 4].as_ref()));
    assert_eq!(c.next(), Some(vec![2, 3, 4].as_ref()));
    assert_eq!(c.next(), None);
    assert_eq!(c.next(), None);
}
