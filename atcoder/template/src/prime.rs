
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

fn prime(n: usize) -> Vec<usize>
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

    let mut ps = Vec::new();
    for (i, f) in pf.iter().enumerate() {
        let p = i + 2;
        if *f {
            ps.push(p);
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

    let ps = prime(30);
    assert_eq!(ps, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}
