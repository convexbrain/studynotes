//#############################################################################

#[derive(Debug, Clone)]
struct ProdIter<I: Iterator>
{
    iters: Vec<I>,
}

#[derive(Debug, Clone)]
struct IterProd<'a, I: Iterator>
{
    iters_org: &'a[I],
    iters: Vec<I>,
    ret: Vec<I::Item>,
}

impl<I: Iterator + Clone> ProdIter<I>
{
    fn new() -> Self {
        ProdIter {
            iters: Vec::new(),
        }
    }

    fn push(&mut self, iter: I) {
        self.iters.push(iter);
    }

    fn iter(&self) -> IterProd<'_, I> {
        IterProd {
            iters_org: &self.iters,
            iters: self.iters.clone(),
            ret: Vec::new(),
        }
    }
}

impl<'a, I: Iterator + Clone> Iterator for IterProd<'a, I>
{
    type Item = &'a[I::Item];

    fn next(&mut self) -> Option<Self::Item> {
        if self.ret.len() == 0 {
            for pos in 0..self.iters.len() {
                if let Some(t) = self.iters[pos].next() {
                    self.ret.push(t);
                }
                else {
                    return None;
                }
            }

            Some(
                unsafe {
                    // self is borrowed as `&mut`, but this returns its contents as `&`.
                    // It violates `&mut` constraints.
                    std::mem::transmute(self.ret.as_slice())
                }
            )
        }
        else {
            for pos in (0..self.iters.len()).rev() {
                if let Some(t) = self.iters[pos].next() {
                    self.ret[pos] = t;
                    return Some(
                        unsafe {
                            // self is borrowed as `&mut`, but this returns its contents as `&`.
                            // It violates `&mut` constraints.
                            std::mem::transmute(self.ret.as_slice())
                        }
                    );
                }
                else {
                    self.iters[pos] = self.iters_org[pos].clone();
                    self.ret[pos] = self.iters[pos].next().unwrap();
                }
            }
            None
        }
    }
}

//#############################################################################

#[test]
fn test_prod_iter() {
    let e = [
        [0, 2, 4],
        [0, 2, 5],
        [0, 3, 4],
        [0, 3, 5],
        [1, 2, 4],
        [1, 2, 5],
        [1, 3, 4],
        [1, 3, 5],
    ];

    let mut p = ProdIter::new();
    p.push(0..2);
    p.push(2..4);
    p.push(4..6);

    for (i, v) in p.iter().enumerate() {
        assert_eq!(v, e[i]);
    }
}
