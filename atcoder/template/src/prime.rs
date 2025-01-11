
//#############################################################################

fn isqrt(n: usize) -> usize
{
    if n < 2 {
        n
    }
    else {
        let mut l = 1;
        let mut r = n;
        while l + 1 < r {
            let c = (l + r) / 2;
            if c * c > n {
                r = c;
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
            ps.extend([p.try_into().unwrap()]);
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

    let ps: std::collections::BTreeSet<u32> = prime(30);
    let ps: Vec<u32> = ps.iter().copied().collect();
    assert_eq!(ps, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}
