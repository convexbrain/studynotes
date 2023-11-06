use std::cmp::*;

//#############################################################################

#[derive(Clone)]
struct AnOrd<V, C> {
    v: V,
    compare: C,
}

impl<V, C> AnOrd<V, C>
where C: Copy + Fn(&V, &V) -> Ordering
{
    fn def(v: V, compare: C) -> Self {
        AnOrd {v, compare}
    }

    fn cp(&self, v: V) -> Self {
        AnOrd {v, compare: self.compare}
    }
}

impl<V, C> std::fmt::Debug for AnOrd<V, C>
where V: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.v.fmt(f)
    }
}

impl<V: PartialEq, C> PartialEq for AnOrd<V, C> {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl<V: PartialEq, C> Eq for AnOrd<V, C> {}

impl<V: PartialEq, C> PartialOrd for AnOrd<V, C>
where C: Fn(&V, &V) -> Ordering
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.compare)(&self.v, &other.v))
    }
}

impl<V: Eq, C> Ord for AnOrd<V, C>
where C: Fn(&V, &V) -> Ordering
{
    fn cmp(&self, other: &Self) -> Ordering {
        (self.compare)(&self.v, &other.v)
    }
}

//#############################################################################

#[test]
fn test_an_ord() {
    use std::collections::BinaryHeap;

    let mut bh = BinaryHeap::new();

    let def = AnOrd::def(0_i32, |a, b| a.abs().cmp(&b.abs()));

    bh.push(def.cp(-5));
    bh.push(def.cp(-3));
    bh.push(def.cp(-1));
    bh.push(def.cp(0));
    bh.push(def.cp(2));
    bh.push(def.cp(4));

    assert_eq!(bh.pop().unwrap().v, -5);
    assert_eq!(bh.pop().unwrap().v, 4);
    assert_eq!(bh.pop().unwrap().v, -3);
    assert_eq!(bh.pop().unwrap().v, 2);
    assert_eq!(bh.pop().unwrap().v, -1);
    assert_eq!(bh.pop().unwrap().v, 0);
    assert_eq!(bh.pop(), None);
    assert_eq!(std::mem::size_of_val(&def), std::mem::size_of_val(&def.v));
}