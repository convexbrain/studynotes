type MatFP = f64;
const MATFP_EPSILON: MatFP = std::f64::EPSILON;

#[derive(Debug)]
struct Mat
{
    nrows: usize,
    ncols: usize,
    //
    stride: usize,
    transposed: bool,
    //
    vec: Vec<MatFP>
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
    fn _get(&self, r: usize, c: usize) -> MatFP
    {
        self.vec[self.stride * c + r]
    }
    //
    pub fn get(&self, r: usize, c: usize) -> MatFP
    {
        if !self.transposed {
            self._get(r, c)
        }
        else {
            self._get(c, r)
        }
    }
    //
    pub fn _set(&mut self, r: usize, c: usize, v: MatFP)
    {
        self.vec[self.stride * c + r] = v;
    }
    //
    pub fn set(&mut self, r: usize, c: usize, v: MatFP)
    {
        if !self.transposed {
            self._set(r, c, v);
        }
        else {
            self._set(c, r, v);
        }
    }
    //
    pub fn set_by<F>(&mut self, f: F)
    where F: Fn() -> MatFP
    {
        for r in 0 .. self.nrows {
            for c in 0 .. self.ncols {
                self.set(r, c, f());
            }
        }
    }
    //
    pub fn set_identity(&mut self)
    {
        for r in 0 .. self.nrows {
            for c in 0 .. self.ncols {
                self.set(r, c, if r == c {1.0} else {0.0});
            }
        }
    }
    //
    pub fn dim(&self) -> (usize, usize)
    {
        if !self.transposed {
            (self.nrows, self.ncols)
        }
        else {
            (self.ncols, self.nrows)
        }
    }
    //
    pub fn t(mut self) -> Mat
    {
        self.transposed = !self.transposed;
        self
    }
    //
    pub fn sq_norm(&self) -> MatFP
    {
        let mut a: MatFP = 0.0;

        for r in 0 .. self.nrows {
            for c in 0 .. self.ncols {
                let v = self.get(r, c);
                a += v * v;
            }
        }

        a
    }
}

impl std::ops::Mul for Mat
{
    type Output = Mat;

    fn mul(self, rhs: Mat) -> Mat
    {
        let (l_nrows, l_ncols) = self.dim();
        let (r_nrows, r_ncols) = rhs.dim();

        assert_eq!(l_ncols, r_nrows);

        let mut mat = Mat::new(l_nrows, r_ncols);

        for r in 0 .. l_nrows {
            for c in 0 .. r_ncols {
                let mut v: MatFP = 0.0;
                for k in 0 .. l_ncols {
                    v += self.get(r, k) * rhs.get(k, c);
                }
                mat.set(r, c, v);
            }
        }

        mat
    }
}

//

const TOL_CNV2: MatFP = MATFP_EPSILON * MATFP_EPSILON;

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
            s: Mat::new(u_ncols, 1),
            v: Mat::new(u_ncols, u_ncols)
        };

        svd.v.set_identity(); // TODO: re-initialize

        svd
    }
    //
    fn apply_jacobi_rot(&mut self, c1: usize, c2: usize) -> bool
    {
        let a = self.u.col(c1).sq_norm();
        let b = self.u.col(c2).sq_norm();
        let d = (self.u.col(c1) * self.u.col(c2).t()).get(0, 0);

        let converged = d * d <= TOL_CNV2 * a * b;

        if converged {
            let zeta = (b - a) / (2.0 * d);
            let t = if zeta > 0.0 {
                1.0 / (zeta + MatFP::sqrt(1.0 + zeta * zeta))
            }
            else {
                -1.0 / (-zeta + MatFP::sqrt(1.0 + zeta * zeta))
            };
            let c = 1.0 / MatFP::sqrt(1.0 + t * t);
            let s = c * t;

            let tmp = self.u.col(c1);
            self.u.col(c1) = c * tmp - s * self.u.col(c2);
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
