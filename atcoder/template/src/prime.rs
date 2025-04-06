use std::ops::*;

//#############################################################################

fn isqrt<N, F>(n: N, checked_square: F) -> N
where N: Default + Copy + Add<Output=N> + Div<Output=N> + PartialOrd<N>,
      F: FnOnce(N) -> Option<N> + Copy
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
            if let Some(cc) = checked_square(c) {
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

fn prime<N, V>(n: N) -> V
where N: TryInto<usize> + TryFrom<usize>,
      <N as TryInto<usize>>::Error: std::fmt::Debug,
      <N as TryFrom<usize>>::Error: std::fmt::Debug,
      V: Extend<N> + Default
{
    let n: usize = n.try_into().unwrap();
    let n_isqrt = isqrt(n, |x| x.checked_mul(x));

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
            ps.extend([p.try_into().unwrap()]);
        }
    }
    ps
}

//#############################################################################

#[test]
fn test_prime() {
    assert_eq!(isqrt(35_u32, |x| x.checked_mul(x)), 5);
    assert_eq!(isqrt(36_u32, |x| x.checked_mul(x)), 6);
    assert_eq!(isqrt(37_u32, |x| x.checked_mul(x)), 6);

    let ps: std::collections::BTreeSet<u32> = prime(30);
    let ps: Vec<u32> = ps.iter().copied().collect();
    assert_eq!(ps, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}
