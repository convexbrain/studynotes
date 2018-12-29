type FP = f64;
const FP_EPSILON: FP = std::f64::EPSILON;

#[derive(Debug)]
struct Mat
{
    nrows: usize,
    ncols: usize,
    //
    stride: usize,
    transposed: bool,
    //
    vec: Vec<FP>
}

impl Mat
{
    pub fn new(nrows: usize, ncols: usize) -> Mat
    {
        Mat {
            nrows,
            ncols,
            stride: nrows,
            transposed: false,
            vec: vec![0.0; nrows * ncols]
        }
    }
    //
    pub fn new_vec(nrows: usize) -> Mat
    {
        Mat::new(nrows, 1)
    }
    //
    pub fn col(&self, c: usize) -> Mat
    {
        assert!(!self.transposed); // TODO: transposed

        Mat {
            nrows: self.nrows,
            ncols: 1,
            stride: self.nrows,
            transposed: false,
            vec: self.vec[self.stride * c .. self.stride * (c + 1)].to_vec() // TODO: copy-less
        }
    }
    //
    pub fn set_by<F>(&mut self, f: F)
    where F: Fn() -> FP
    {
        for r in 0 .. self.nrows {
            for c in 0 .. self.ncols {
                self[(r, c)] = f();
            }
        }
    }
    //
    pub fn set_identity(&mut self)
    {
        for r in 0 .. self.nrows {
            for c in 0 .. self.ncols {
                self[(r, c)] = if r == c {1.0} else {0.0};
            }
        }
    }
    //
    pub fn t(mut self) -> Mat
    {
        self.transposed = !self.transposed;
        self
    }
}

impl std::ops::Index<(usize, usize)> for Mat
{
    type Output = FP;
    fn index(&self, index: (usize, usize)) -> &FP
    {
        if !self.transposed {
            &self.vec[self.stride * index.1 + index.0]
        }
        else {
            &self.vec[self.stride * index.0 + index.1]
        }
    }
}

impl std::ops::IndexMut<(usize, usize)> for Mat
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut FP
    {
        if !self.transposed {
            &mut self.vec[self.stride * index.1 + index.0]
        }
        else {
            &mut self.vec[self.stride * index.0 + index.1]
        }
    }
}

trait MatOps
{
    fn is_mat(&self) -> bool
    {
        false
    }
    fn dim(&self) -> (usize, usize)
    {
        (1, 1)
    }
    fn get(&self, row: usize, col: usize) -> FP;
    fn set(&mut self, row: usize, col: usize, value: FP);
}

impl MatOps for Mat
{
    fn is_mat(&self) -> bool
    {
        true
    }
    //
    fn dim(&self) -> (usize, usize)
    {
        if !self.transposed {
            (self.nrows, self.ncols)
        }
        else {
            (self.ncols, self.nrows)
        }
    }
    //
    fn get(&self, row: usize, col: usize) -> FP
    {
        self[(row, col)]
    }
    //
    fn set(&mut self, row: usize, col: usize, value: FP)
    {
        self[(row, col)] = value;
    }
}

impl MatOps for FP
{
    fn get(&self, _: usize, _: usize) -> FP
    {
        *self
    }
    //
    fn set(&mut self, _: usize, _: usize, value: FP)
    {
        *self = value;
    }
}

impl<T> std::ops::Mul<T> for Mat
where T: MatOps
{
    type Output = Mat;

    fn mul(self, rhs: T) -> Mat
    {
        let (l_nrows, l_ncols) = self.dim();

        if rhs.is_mat() {
            let (r_nrows, r_ncols) = rhs.dim();

            assert_eq!(l_ncols, r_nrows);

            let mut mat = Mat::new(l_nrows, r_ncols); // TODO: new-less

            for r in 0 .. l_nrows {
                for c in 0 .. r_ncols {
                    let mut v: FP = 0.0;
                    for k in 0 .. l_ncols {
                        v += self[(r, k)] * rhs.get(k, c);
                    }
                    mat[(r, c)] = v;
                }
            }

            mat
        }
        else {
            let mut mat = Mat::new(l_nrows, l_ncols); // TODO: new-less

            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    mat[(r, c)] = self[(r, c)] * rhs.get(0, 0);
                }
            }

            mat
        }
    }
}

impl<T> std::ops::Add<T> for Mat
where T: MatOps
{
    type Output = Mat;

    fn add(self, rhs: T) -> Mat
    {
        let (l_nrows, l_ncols) = self.dim();

        if rhs.is_mat() {
            let (r_nrows, r_ncols) = rhs.dim();

            assert_eq!(l_nrows, r_nrows);
            assert_eq!(l_ncols, r_ncols);

            let mut mat = Mat::new(l_nrows, l_ncols); // TODO: new-less

            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    mat[(r, c)] = self[(r, c)] + rhs.get(r, c);
                }
            }

            mat
        }
        else {
            let mut mat = Mat::new(l_nrows, l_ncols); // TODO: new-less

            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    mat[(r, c)] = self[(r, c)] + rhs.get(0, 0);
                }
            }

            mat
        }
    }
}

impl<T> std::ops::Sub<T> for Mat
where T: MatOps
{
    type Output = Mat;

    fn sub(self, rhs: T) -> Mat
    {
        let (l_nrows, l_ncols) = self.dim();

        if rhs.is_mat() {
            let (r_nrows, r_ncols) = rhs.dim();

            assert_eq!(l_nrows, r_nrows);
            assert_eq!(l_ncols, r_ncols);

            let mut mat = Mat::new(l_nrows, l_ncols); // TODO: new-less

            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    mat[(r, c)] = self[(r, c)] - rhs.get(r, c);
                }
            }

            mat
        }
        else {
            let mut mat = Mat::new(l_nrows, l_ncols); // TODO: new-less

            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    mat[(r, c)] = self[(r, c)] - rhs.get(0, 0);
                }
            }

            mat
        }
    }
}


//

const TOL_CNV2: FP = FP_EPSILON * FP_EPSILON;

#[derive(Debug)]
struct MatSVD
{
    transposed: bool,
    //
    u: Mat,
    s: Mat,
    v: Mat
}

impl MatSVD
{
    pub fn new(g: Mat) -> MatSVD
    {
        let (nrows, ncols) = g.dim();
        let transposed = nrows < ncols;

        let (_u_nrows, u_ncols) = if !transposed {
            (nrows, ncols)
        }
        else {
            (ncols, nrows)
        };

        let mut svd = MatSVD {
            transposed,
            u: if !transposed {g} else {g.t()}, // TODO: re-initialize
            s: Mat::new_vec(u_ncols),
            v: Mat::new(u_ncols, u_ncols)
        };

        svd.v.set_identity(); // TODO: re-initialize

        svd
    }
    //
    fn apply_jacobi_rot(&mut self, c1: usize, c2: usize) -> bool
    {
        let a = (self.u.col(c1).t() * self.u.col(c1))[(0, 0)];
        let b = (self.u.col(c2).t() * self.u.col(c2))[(0, 0)];
        let d = (self.u.col(c1).t() * self.u.col(c2))[(0, 0)];

        let converged = d * d <= TOL_CNV2 * a * b;

        if !converged {
            let zeta = (b - a) / (2.0 * d);
            let t = if zeta > 0.0 {
                1.0 / (zeta + FP::sqrt(1.0 + zeta * zeta))
            }
            else {
                -1.0 / (-zeta + FP::sqrt(1.0 + zeta * zeta))
            };
            let c = 1.0 / FP::sqrt(1.0 + t * t);
            let s = c * t;

            let tmp = self.u.col(c1);
            let ttt = tmp * c - self.u.col(c2) * s;
            println!("{:?}", ttt);
            //self.u.col(c1) = c * tmp - s * self.u.col(c2);
        }
        panic!();

        converged
    }
    //
    fn norm_singular(&mut self)
    {
        panic!();
    }
    //
    pub fn do_decomp(&mut self)
    {
        let (m, n) = self.u.dim();

        let mut converged_all = false;
        while !converged_all {
            converged_all = true;

            for i in 0 .. n - 1 {
                for j in i + 1 .. n {
                    if !self.apply_jacobi_rot(i, j) {converged_all = false;}
                }
            }
        }

        self.norm_singular();
    }
}

//

fn main()
{
    let mut mat = Mat::new(4, 4);
    mat.set_by(|| {rand::random()});
    println!("{:?}", mat);
    println!();

    let mut svd = MatSVD::new(mat);
    println!("{:?}", svd);
    println!();

    svd.do_decomp();
    println!("{:?}", svd);
    println!();
}
