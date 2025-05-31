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

fn mod_pow<N: Int>(mut x: N, mut p: N, m: N) -> N
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

//#############################################################################

#[test]
fn test_mod_pow() {
    assert_eq!(mod_pow(238456, 27564, 923453876_u64), 706933036);
}
