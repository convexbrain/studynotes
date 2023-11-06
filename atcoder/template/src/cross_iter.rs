//#############################################################################

#[derive(Debug, Clone)]
struct CrossIter<I0: Iterator, I1: Iterator>
{
    iters: (I0, I1)
}

#[derive(Debug, Clone)]
struct IterCross<'a, I0: Iterator, I1: Iterator>
{
    iters_org: &'a (I0, I1),
    iters: (I0, I1),
    ret: Option<(I0::Item, I1::Item)>,
}

impl<I0: Iterator + Clone, I1: Iterator + Clone> CrossIter<I0, I1>
{
    fn new(iter0: I0, iter1: I1) -> Self {
        CrossIter {
            iters: (iter0, iter1),
        }
    }

    fn iter(&self) -> IterCross<'_, I0, I1> {
        IterCross {
            iters_org: &self.iters,
            iters: self.iters.clone(),
            ret: None,
        }
    }
}

impl<'a, I0: Iterator + Clone, I1: Iterator + Clone> Iterator for IterCross<'a, I0, I1>
{
    type Item = &'a (I0::Item, I1::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t1) = self.iters.1.next() {
            if self.ret.is_some() {
                let (t0, _) = self.ret.take().unwrap();
                self.ret = Some((t0, t1));
            }
            else if let Some(t0) = self.iters.0.next() {
                self.ret = Some((t0, t1))
            }
            else {
                self.ret = None
            }
        }
        else {
            if let Some(t0) = self.iters.0.next() {
                self.iters.1 = self.iters_org.1.clone();
                if let Some(t1) = self.iters.1.next() {
                    self.ret = Some((t0, t1))
                }
                else {
                    self.ret = None
                }
            }
            else {
                self.ret = None
            }

        }

        if self.ret.is_some() {
            unsafe {
                // self is borrowed as `&mut`, but this returns its contents as `&`.
                // It violates `&mut` constraints.
                std::mem::transmute(self.ret.as_ref())
            }
        }
        else {
            None
        }
    }
}

//#############################################################################

#[test]
fn test_cross_iter() {
    let e = [
        ('a', 0),
        ('a', 1),
        ('a', 2),
        ('b', 0),
        ('b', 1),
        ('b', 2),
        ('c', 0),
        ('c', 1),
        ('c', 2),
    ];

    for (i, v) in CrossIter::new('a'..='c', 0..3).iter().enumerate() {
        assert_eq!(v, &e[i]);
    }
}
