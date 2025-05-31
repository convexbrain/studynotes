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

fn isqrt<N: Int>(n: N) -> N
{
    let zero = N::default();
    if n == zero {
        zero
    }
    else {
        let one = n / n;
        let two = one + one;
        let mut l = one;
        let mut r = n;
        while l + one < r {
            let c = (l + r) / two;
            if let Some(cc) = c.chk_mul(c) {
                if cc > n {
                    r = c;
                }
                else {
                    l = c;
                }
            }
            else {
                l = c;
            }
        }

        l
    }
}

fn prime<V>(n: usize) -> V
where V: Extend<usize> + Default
{
    let n_isqrt = isqrt(n);

    let mut pf = vec![true; n - 1]; // 0..n-1 == 0..=n-2 == 2..=n
    for p in 2..=n_isqrt {
        if pf[p - 2] {
            for k in 2.. {
                let j = p * k;
                if j <= n {
                    pf[j - 2] = false;
                }
                else {
                    break;
                }
            }
        }
    }

    let mut ps = V::default();
    for (i, f) in pf.iter().enumerate() {
        let p = i + 2;
        if *f {
            ps.extend([p.try_into().unwrap_or_default()]);
        }
    }
    ps
}

//#############################################################################

#[test]
fn test_prime() {
    assert_eq!(isqrt(35), 5);
    assert_eq!(isqrt(36), 6);
    assert_eq!(isqrt(37), 6);

    let ps: std::collections::BTreeSet<_> = prime(30);
    let ps: Vec<_> = ps.iter().copied().collect();
    assert_eq!(ps, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}
