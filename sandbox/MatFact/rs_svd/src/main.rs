type FP = f64;
const FP_EPSILON: FP = std::f64::EPSILON;

#[derive(Debug)]
enum View<'a>
{
    Own(Vec<FP>),
    Borrow(&'a [FP]),
    BorrowMut(&'a mut[FP])
}

impl<'a> Clone for View<'a>
{
    fn clone(&self) -> Self
    {
        match &self {
            View::Own(v) => View::Own(v.clone()),
            View::Borrow(v) => View::Own(v.to_vec()),
            View::BorrowMut(v) => View::Own(v.to_vec())
        }
    }
}

#[derive(Debug, Clone)]
struct Mat<'a>
{
    nrows: usize,
    ncols: usize,
    //
    offset: usize,
    stride: usize,
    //
    transposed: bool,
    //
    view: View<'a>
}

impl<'a> Mat<'a>
{
    pub fn new(nrows: usize, ncols: usize) -> Mat<'a>
    {
        Mat {
            nrows,
            ncols,
            offset: 0,
            stride: nrows,
            transposed: false,
            view: View::Own(vec![0.0; nrows * ncols])
        }
    }
    //
    pub fn new_vec(nrows: usize) -> Mat<'a>
    {
        Mat::new(nrows, 1)
    }
    //
    fn tr_bound<RR, CR>(&self, rows: RR, cols: CR) -> (std::ops::Range<usize>, std::ops::Range<usize>)
    where RR: std::ops::RangeBounds<usize>, CR: std::ops::RangeBounds<usize>
    {
        let row_b = match rows.start_bound() {
            std::ops::Bound::Unbounded => 0,
            std::ops::Bound::Included(&i) => i,
            std::ops::Bound::Excluded(&i) => i + 1
        };

        let row_e = match rows.end_bound() {
            std::ops::Bound::Unbounded => if !self.transposed {self.nrows} else {self.ncols},
            std::ops::Bound::Included(&i) => i - 1,
            std::ops::Bound::Excluded(&i) => i
        };

        let col_b = match cols.start_bound() {
            std::ops::Bound::Unbounded => 0,
            std::ops::Bound::Included(&i) => i,
            std::ops::Bound::Excluded(&i) => i + 1
        };

        let col_e = match cols.end_bound() {
            std::ops::Bound::Unbounded => if !self.transposed {self.ncols} else {self.nrows},
            std::ops::Bound::Included(&i) => i - 1,
            std::ops::Bound::Excluded(&i) => i
        };

        if !self.transposed {
            (std::ops::Range{start: row_b, end: row_e}, std::ops::Range{start: col_b, end: col_e})
        }
        else {
            (std::ops::Range{start: col_b, end: col_e}, std::ops::Range{start: row_b, end: row_e})
        }
    }
    //
    pub fn slice<RR, CR>(&self, rows: RR, cols: CR) -> Mat
    where RR: std::ops::RangeBounds<usize>,  CR: std::ops::RangeBounds<usize>
    {
        let (row_range, col_range) = self.tr_bound(rows, cols);

        let view = match &self.view {
            View::Own(v) => View::Borrow(&v),
            View::Borrow(v) => View::Borrow(&v),
            View::BorrowMut(v) => View::Borrow(&v),
        };

        Mat {
            nrows: row_range.end - row_range.start,
            ncols: col_range.end - col_range.start,
            offset: self.offset + self.stride * col_range.start + row_range.start,
            view,
            .. *self
        }
    }
    //
    pub fn slice_mut<RR, CR>(&mut self, rows: RR, cols: CR) -> Mat
    where RR: std::ops::RangeBounds<usize>,  CR: std::ops::RangeBounds<usize>
    {
        let (row_range, col_range) = self.tr_bound(rows, cols);

        let view = match &mut self.view {
            View::Own(v) => View::BorrowMut(v),
            View::Borrow(_) => panic!("cannot borrow immutable as mutable"),
            View::BorrowMut(v) => View::BorrowMut(v),
        };

        Mat {
            nrows: row_range.end - row_range.start,
            ncols: col_range.end - col_range.start,
            offset: self.offset + self.stride * col_range.start + row_range.start,
            view,
            .. *self
        }
    }
    //
    pub fn row(&self, r: usize) -> Mat
    {
        self.slice(r .. r + 1, ..)
    }
    //
    pub fn col(&self, c: usize) -> Mat
    {
        self.slice(.., c .. c + 1)
    }
    //
    pub fn row_mut(&mut self, r: usize) -> Mat
    {
        self.slice_mut(r .. r + 1, ..)
    }
    //
    pub fn col_mut(&mut self, c: usize) -> Mat
    {
        self.slice_mut(.., c .. c + 1)
    }
    //
    fn tr_index(&self, index: (usize, usize)) -> usize
    {
        if !self.transposed {
            self.offset + self.stride * index.1 + index.0
        }
        else {
            self.offset + self.stride * index.0 + index.1
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
    pub fn assign(&mut self, rhs: Mat)
    {
        let (l_nrows, l_ncols) = self.dim();
        let (r_nrows, r_ncols) = rhs.dim();

        assert_eq!(l_nrows, r_nrows);
        assert_eq!(l_ncols, r_ncols);
        
        for r in 0 .. self.nrows {
            for c in 0 .. self.ncols {
                self[(r, c)] = rhs[(r, c)];
            }
        }
    }
    //
    pub fn t(mut self) -> Mat<'a>
    {
        self.transposed = !self.transposed;
        self
    }
}

impl<'a> std::ops::Index<(usize, usize)> for Mat<'a>
{
    type Output = FP;
    fn index(&self, index: (usize, usize)) -> &FP
    {
        let i = self.tr_index(index);

        match &self.view {
            View::Own(v) => &v[i],
            View::Borrow(v) => &v[i],
            View::BorrowMut(v) => &v[i]
        }
    }
}

impl<'a> std::ops::IndexMut<(usize, usize)> for Mat<'a>
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut FP
    {
        let i = self.tr_index(index);

        match &mut self.view {
            View::Own(v) => &mut v[i],
            View::Borrow(_) => panic!("cannot borrow immutable as mutable"),
            View::BorrowMut(v) => &mut v[i]
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

impl<'a> MatOps for Mat<'a>
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

impl<'a, T> std::ops::Mul<T> for Mat<'a>
where T: MatOps
{
    type Output = Mat<'a>;

    fn mul(self, rhs: T) -> Mat<'a>
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

impl<'a, T> std::ops::Add<T> for Mat<'a>
where T: MatOps
{
    type Output = Mat<'a>;

    fn add(self, rhs: T) -> Mat<'a>
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

impl<'a, T> std::ops::Sub<T> for Mat<'a>
where T: MatOps
{
    type Output = Mat<'a>;

    fn sub(self, rhs: T) -> Mat<'a>
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

impl<'a, T> std::ops::MulAssign<T> for Mat<'a>
where T: MatOps
{
    fn mul_assign(&mut self, rhs: T) {
        let (l_nrows, l_ncols) = self.dim();

        if rhs.is_mat() {
            panic!("not implemented");
        }
        else {
            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    self[(r, c)] *= rhs.get(0, 0);
                }
            }
        }
    }
}

impl<'a, T> std::ops::AddAssign<T> for Mat<'a>
where T: MatOps
{
    fn add_assign(&mut self, rhs: T) {
        let (l_nrows, l_ncols) = self.dim();

        if rhs.is_mat() {
            let (r_nrows, r_ncols) = rhs.dim();

            assert_eq!(l_nrows, r_nrows);
            assert_eq!(l_ncols, r_ncols);

            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    self[(r, c)] += rhs.get(r, c);
                }
            }
        }
        else {
            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    self[(r, c)] += rhs.get(0, 0);
                }
            }
        }
    }
}

impl<'a, T> std::ops::SubAssign<T> for Mat<'a>
where T: MatOps
{
    fn sub_assign(&mut self, rhs: T) {
        let (l_nrows, l_ncols) = self.dim();

        if rhs.is_mat() {
            let (r_nrows, r_ncols) = rhs.dim();

            assert_eq!(l_nrows, r_nrows);
            assert_eq!(l_ncols, r_ncols);

            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    self[(r, c)] -= rhs.get(r, c);
                }
            }
        }
        else {
            for r in 0 .. l_nrows {
                for c in 0 .. l_ncols {
                    self[(r, c)] -= rhs.get(0, 0);
                }
            }
        }
    }
}

//

const TOL_CNV2: FP = FP_EPSILON * FP_EPSILON;

#[derive(Debug)]
struct MatSVD<'a>
{
    transposed: bool,
    //
    u: Mat<'a>,
    s: Mat<'a>,
    v: Mat<'a>
}

impl<'a> MatSVD<'a>
{
    pub fn new(g: Mat<'a>) -> MatSVD<'a>
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

            let (r, _) = self.u.dim();
            let tmp1 = Mat::new_vec(r) + self.u.col(c1) * c - self.u.col(c2) * s;
            println!("{:?}", tmp1);
            println!("{:?}", self.u);
            self.u.col_mut(c1).assign(tmp1);
            println!("{:?}", self.u);
        }
        panic!("not implemented");
        
        converged
    }
    //
    fn norm_singular(&mut self)
    {
        panic!("not implemented");
    }
    //
    pub fn do_decomp(&mut self)
    {
        let (_, n) = self.u.dim();

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
