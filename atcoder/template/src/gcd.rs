use std::ops::*;

trait Int: std::fmt::Debug + Copy + Default + Ord + Eq + ShrAssign + SubAssign +
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> + BitAnd<Output=Self> +
    TryInto<usize> + TryFrom<usize>
{ fn chk_mul(self, rhs: Self) -> Option<Self>; }
impl Int for u8 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u16 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u32 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u64 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for u128 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for usize { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i8 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i16 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i32 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i64 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for i128 { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }
impl Int for isize { fn chk_mul(self, rhs: Self) -> Option<Self> {self.checked_mul(rhs)} }

//#############################################################################

fn gcd<N: Int>(mut m: N, mut n: N) -> N
{
    if m < n {
        (m, n) = (n, m);
    }
    while n != N::default() {
        (m, n) = (n, m % n);
    }
    return m;
}

//#############################################################################

#[test]
fn test_gcd() {
    assert_eq!(gcd(48000, 44100), 300);
}
