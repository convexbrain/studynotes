use std::ops::*;

//#############################################################################

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

//#############################################################################

#[test]
fn test_gcd() {
    assert_eq!(gcd(48000, 44100), 300);
}
