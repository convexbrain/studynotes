type FP = f64;
use std::f64::EPSILON as FP_EPSILON;
use std::f64::MIN as FP_MIN;

use std::cmp::PartialEq;
use std::ops::{Range, RangeBounds, Bound};
use std::ops::{Add, Index, IndexMut};

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
    diagonal: bool,
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
            diagonal: false,
            view: View::Own(vec![0.0; nrows * ncols])
        }
    }
    //
    pub fn new1(nrows: usize) -> Mat<'a>
    {
        Mat::new(nrows, 1)
    }
    //
    fn tr_bound<RR, CR>(&self, rows: RR, cols: CR) -> (Range<usize>, Range<usize>)
    where RR: RangeBounds<usize>, CR: RangeBounds<usize>
    {
        let row_b = match rows.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1
        };

        let row_e = match rows.end_bound() {
            Bound::Unbounded => if !self.transposed {self.nrows} else {self.ncols},
            Bound::Included(&i) => i - 1,
            Bound::Excluded(&i) => i
        };

        let col_b = match cols.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1
        };

        let col_e = match cols.end_bound() {
            Bound::Unbounded => if !self.transposed {self.ncols} else {self.nrows},
            Bound::Included(&i) => i - 1,
            Bound::Excluded(&i) => i
        };

        if !self.transposed {
            (Range{start: row_b, end: row_e}, Range{start: col_b, end: col_e})
        }
        else {
            (Range{start: col_b, end: col_e}, Range{start: row_b, end: row_e})
        }
    }
    //
    pub fn slice<RR, CR>(&self, rows: RR, cols: CR) -> Mat
    where RR: RangeBounds<usize>,  CR: RangeBounds<usize>
    {
        let (row_range, col_range) = self.tr_bound(rows, cols);

        let view = match &self.view {
            View::Own(v) => View::Borrow(&v),
            View::Borrow(v) => View::Borrow(&v),
            View::BorrowMut(v) => View::Borrow(&v)
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
    where RR: RangeBounds<usize>,  CR: RangeBounds<usize>
    {
        let (row_range, col_range) = self.tr_bound(rows, cols);

        let view = match &mut self.view {
            View::Own(v) => View::BorrowMut(v),
            View::Borrow(_) => panic!("cannot convert Borrow to BorrowMut"),
            View::BorrowMut(v) => View::BorrowMut(v)
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
        self.slice(r..=r, ..)
    }
    //
    pub fn col(&self, c: usize) -> Mat
    {
        self.slice(.., c..=c)
    }
    //
    pub fn row_mut(&mut self, r: usize) -> Mat
    {
        self.slice_mut(r..=r, ..)
    }
    //
    pub fn col_mut(&mut self, c: usize) -> Mat
    {
        self.slice_mut(.., c..=c)
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
    pub fn set_by<F>(mut self, f: F) -> Mat<'a>
    where F: Fn(usize, usize) -> FP
    {
        for c in 0 .. self.ncols {
            for r in 0 .. self.nrows {
                self[(r, c)] = f(r, c);
            }
        }
        self
    }
    //
    pub fn set_eye(mut self) -> Mat<'a>
    {
        for c in 0 .. self.ncols {
            for r in 0 .. self.nrows {
                self[(r, c)] = if r == c {1.} else {0.};
            }
        }
        self
    }
    //
    pub fn set_iter<'b, T>(mut self, iter: T) -> Mat<'a>
    where T: IntoIterator<Item=&'b FP>
    {
        // NOTE: read row-wise
        let mut i = iter.into_iter();
        for c in 0 .. self.ncols {
            for r in 0 .. self.nrows {
                self[(r, c)] = *i.next().unwrap_or(&0.);
            }
        }
        self
    }
    //
    pub fn assign(&mut self, rhs: &Mat)
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
    pub fn t(&self) -> Mat
    {
        let view = match &self.view {
            View::Own(v) => View::Borrow(&v),
            View::Borrow(v) => View::Borrow(&v),
            View::BorrowMut(v) => View::Borrow(&v)
        };

        Mat {
            transposed: !self.transposed,
            view,
            .. *self
        }
    }
    //
    pub fn mul_diag(&self, rhs: &Mat) -> Mat
    {
        let (l_nrows, l_ncols) = self.dim();

        assert_eq!((l_ncols, 1), rhs.dim());

        let mut mat = Mat::new(l_nrows, l_ncols);

        for c in 0 .. l_ncols {
            for r in 0 .. l_nrows {
                mat[(r, c)] = self[(r, c)] * rhs[(c, c)];
            }
        }

        mat
    }
}

//

impl<'a> Index<(usize, usize)> for Mat<'a>
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

impl<'a> IndexMut<(usize, usize)> for Mat<'a>
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut FP
    {
        let i = self.tr_index(index);

        match &mut self.view {
            View::Own(v) => &mut v[i],
            View::Borrow(_) => panic!("cannot index Borrow as mutable"),
            View::BorrowMut(v) => &mut v[i]
        }
    }
}

//

impl<'a> PartialEq for Mat<'a>
{
    fn eq(&self, other: &Self) -> bool
    {
        let (l_nrows, l_ncols) = self.dim();

        if (l_nrows, l_ncols) != other.dim() {
            return false;
        }

        for c in 0 .. l_ncols {
            for r in 0 .. l_nrows {
                if self[(r, c)] != other[(r, c)] {
                    return false;
                }
            }
        }

        true
    }
}

//

trait MatAcc
{
    fn dim(&self) -> (usize, usize);
    fn get(&self, row: usize, col: usize) -> FP;
}

impl<'a> MatAcc for Mat<'a>
{
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
}

impl<'a> MatAcc for &Mat<'a>
{
    fn dim(&self) -> (usize, usize)
    {
        (*self).dim()
    }
    //
    fn get(&self, row: usize, col: usize) -> FP
    {
        (*self).get(row, col)
    }
}

//

impl<'al, T> Add<T> for &Mat<'al>
where T: MatAcc
{
    type Output = Mat<'static>;

    fn add(self, rhs: T) -> Mat<'static>
    {
        let (l_nrows, l_ncols) = self.dim();

        assert_eq!((l_nrows, l_ncols), rhs.dim());

        let mut mat = Mat::new(l_nrows, l_ncols);

        for c in 0 .. l_ncols {
            for r in 0 .. l_nrows {
                mat[(r, c)] = self.get(r, c) + rhs.get(r, c);
            }
        }

        mat
    }
}

impl<'al> Add<FP> for &Mat<'al>
{
    type Output = Mat<'static>;

    fn add(self, rhs: FP) -> Mat<'static>
    {
        let (l_nrows, l_ncols) = self.dim();

        let mut mat = Mat::new(l_nrows, l_ncols);

        for c in 0 .. l_ncols {
            for r in 0 .. l_nrows {
                mat[(r, c)] = self[(r, c)] + rhs;
            }
        }

        mat
    }
}

impl<'al, T> Add<T> for Mat<'al>
where T: MatAcc
{
    type Output = Mat<'static>;

    fn add(self, rhs: T) -> Mat<'static>
    {
        &self + rhs
    }
}

impl<'al> Add<FP> for Mat<'al>
{
    type Output = Mat<'static>;

    fn add(self, rhs: FP) -> Mat<'static>
    {
        &self + rhs
    }
}

impl<'a> Add<&Mat<'a>> for FP
{
    type Output = Mat<'static>;

    fn add(self, rhs: &Mat) -> Mat<'static>
    {
        rhs.add(self)
    }
}

impl<'a> Add<Mat<'a>> for FP
{
    type Output = Mat<'static>;

    fn add(self, rhs: Mat) -> Mat<'static>
    {
        rhs.add(self)
    }
}

// TODO: module, test, display
//

#[test]
fn test()
{
    {
        let a = Mat::new(3, 3).set_eye();
        let b = Mat::new(3, 3).set_iter(&[
            1., 0., 0.,
            0., 1., 0.,
            0., 0., 1.
        ]);
        let c = &a + (&b + &a);
        println!("{:?}", c);
        let c = &a + 1.;
        println!("{:?}", c);
        let c = 1. + &b;
        println!("{:?}", c);
        println!("{:?}", a);
        println!("{:?}", b);
        println!();
    }
    {
        let mut a = Mat::new(3, 3).set_eye();
        let b = Mat::new(3, 3).set_iter(&[
            1., 0., 0.,
            0., 1., 0.,
            0., 0., 1.
        ]);
        let c = &a + &b;
        a.assign(&c);
    }
    {
        let mat = Mat::new1(4).set_by(|_, _| {rand::random()});
        println!("{:?}", mat);
        println!();
    }
    {
        let a = Mat::new(3, 3).set_eye();
        let b = Mat::new(3, 3).set_iter(&[
            1., 0., 0.,
            0., 1., 0.,
            0., 0., 1.
        ]);
        assert_eq!(a, b);
    }
    {
        let a = Mat::new(2, 2).set_by(|r, c| {(r + c) as FP});
        let b = Mat::new(2, 2).set_iter(&[
            0., 1.,
            1., 2.
        ]);
        assert_eq!(a, b);
    }
}
