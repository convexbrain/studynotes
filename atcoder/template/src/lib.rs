use std::ops::*;
use std::collections::*;

///////////////////////////////////////////////////////////////////////////////

fn gcd<N>(mut m: N, mut n: N) -> N
where N: Rem<Output=N> + Ord + Default + Copy
{
    if m < n {
        (m, n) = (n, m);
    }
    while n != N::default() {
        (m, n) = (n, m % n);
    }
    return m;
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(48000, 44100), 300);
}

///////////////////////////////////////////////////////////////////////////////

fn mod_pow<N>(mut x: N, mut p: N, m: N) -> N
where N: Default + Ord + BitAnd<Output=N> + ShrAssign + Mul<Output=N> + Rem<Output=N> + SubAssign + Copy + Div<Output=N>
{
    let zero = N::default();
    let one = m / m;

    if p == zero {
        return one;
    }

    let mut k = one;

    while p > one {
        if p & one == zero {
            x = (x * x) % m;
            p >>= one;
        }
        else {
            k = (k * x) % m;
            p -= one;
        }
    }
    (k * x) % m
}

#[test]
fn test_mod_pow() {
    assert_eq!(mod_pow(238456, 27564, 923453876_u64), 706933036);
}

///////////////////////////////////////////////////////////////////////////////

// (g, x, y) s.t. a x + b y = gcd(a, b) = g
fn ext_euclid<N>(a: N, b: N) -> (N, N, N)
where N: Default + Div<Output=N> + Mul<Output=N> + Sub<Output=N> + Eq + Copy
{
    let zero = N::default();
    let one = b / b;

    let mut r0 = a;
    let mut s0 = one;
    let mut t0 = zero;
    let mut r1 = b;
    let mut s1 = zero;
    let mut t1 = one;

    loop {
        let q1 = r0 / r1;
        let r2 = r0 - q1 * r1;
        let s2 = s0 - q1 * s1;
        let t2 = t0 - q1 * t1;

        if r2 == zero {
            return (r1, s1, t1);
        }

        (r0, r1) = (r1, r2);
        (s0, s1) = (s1, s2);
        (t0, t1) = (t1, t2);
    }
}

fn mod_div<N>(x: N, y: N, m: N) -> N
where N: Default + Div<Output=N> + Mul<Output=N> + Sub<Output=N> + Ord + Copy + Rem<Output=N> + Add<Output=N>
{
    let zero = N::default();
    let one = m / m;

    let (gcd, y_inv, _) = ext_euclid(y, m);

    if gcd != one {panic!();}

    let y_inv = if y_inv < zero {y_inv + m} else {y_inv};

    (x * y_inv) % m
}

#[test]
fn test_ext_euclid() {
    assert_eq!(gcd(48000, 44100), ext_euclid(48000, 44100).0);
}

#[test]
fn test_mod_div() {
    let x = 3187689;
    let y = 7;
    let m = 23341823_i64;

    let z = mod_div(x, y, m);
    assert!(z > 0);
    assert!(z < m);
    assert_ne!(y * z, x);
    assert_eq!((y * z) % m, x);
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Vec2D<T> {
    vec_row_major: Vec<T>,
    nrows: usize,
    ncols: usize,
}

impl<T> Vec2D<T> {
    fn from(vec_row_major: Vec<T>, (nrows, ncols): (usize, usize)) -> Self {
        assert!(vec_row_major.len() >= nrows * ncols);
        Self {vec_row_major, nrows, ncols}
    }

    fn release(self) -> Vec<T> {
        self.vec_row_major
    }
}

impl<T> Vec2D<T> {
    fn get(&self, row: isize, col: isize) -> Option<&T> {
        if row < 0 || col < 0 || !(row < self.nrows as isize) || !(col < self.ncols as isize) {
            None
        }
        else {
            Some(&self.vec_row_major[row as usize * self.ncols + col as usize])
        }
    }

    fn get_mut(&mut self, row: isize, col: isize) -> Option<&mut T> {
        if row < 0 || col < 0 || !(row < self.nrows as isize) || !(col < self.ncols as isize) {
            None
        }
        else {
            Some(&mut self.vec_row_major[row as usize * self.ncols + col as usize])
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.nrows);
        assert!(index.1 < self.ncols);
        &self.vec_row_major[index.0 * self.ncols + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.nrows);
        assert!(index.1 < self.ncols);
        &mut self.vec_row_major[index.0 * self.ncols + index.1]
    }
}

#[test]
fn test_vec2d() {
    let v2 = vec![1, 2, 3,  4, 5, 6];
    let mut v2 = Vec2D::from(v2, (2, 3));
    v2[(1, 2)] = v2[(0, 0)];
    assert_eq!(v2.get(-1, 0), None);
    assert_eq!(v2.get(0, -1), None);
    assert_eq!(v2.get(2, 0), None);
    assert_eq!(v2.get(0, 3), None);
    assert_eq!(v2.release(), [1, 2, 3,  4, 5, 1]);
}

///////////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////////

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
