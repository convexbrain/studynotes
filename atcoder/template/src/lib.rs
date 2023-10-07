use std::ops::*;

//

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

//

struct Vec2D<T> {
    vec: Vec<T>, // column-wise
    nr: usize,
    nc: usize,
}

impl<T> Vec2D<T>
{
    fn new(def: T, (nr, nc): (usize, usize)) -> Self
    where T: Clone
    {
        Self {
            vec: vec![def; nr * nc],
            nr, nc,
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T>
{
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.nr);
        assert!(index.1 < self.nc);
        &self.vec[index.1 * self.nr + index.0]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T>
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.nr);
        assert!(index.1 < self.nc);
        &mut self.vec[index.1 * self.nr + index.0]
    }
}

#[test]
fn test_vec2d() {
    let mut v2 = Vec2D::new(0, (3, 2));
    v2.vec.copy_from_slice(&[1, 2, 3,  4, 5, 6]);
    v2[(1, 1)] = 1;
    assert_eq!(&v2.vec, &[1, 2, 3,  4, 1, 6]);
}

