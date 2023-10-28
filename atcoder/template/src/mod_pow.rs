use std::ops::*;

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
