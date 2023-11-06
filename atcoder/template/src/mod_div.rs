use std::ops::*;

//#############################################################################

// (g, x, y) s.t. a x + b y = gcd(a, b) = g
fn ext_euclid<N>(a: N, b: N) -> (N, N, N)
where N: Default + Div<Output=N> + Mul<Output=N> + Sub<Output=N> + Eq + Copy
{
    let zero = N::default();
    let one = b / b;

    let mut r0 = a;
    let mut s0 = one;
    let mut t0 = zero;
    let mut r1 = b;
    let mut s1 = zero;
    let mut t1 = one;

    loop {
        let q1 = r0 / r1;
        let r2 = r0 - q1 * r1;
        let s2 = s0 - q1 * s1;
        let t2 = t0 - q1 * t1;

        if r2 == zero {
            return (r1, s1, t1);
        }

        (r0, r1) = (r1, r2);
        (s0, s1) = (s1, s2);
        (t0, t1) = (t1, t2);
    }
}

fn mod_div<N>(x: N, y: N, m: N) -> N
where N: Default + Div<Output=N> + Mul<Output=N> + Sub<Output=N> + Ord + Copy + Rem<Output=N> + Add<Output=N>
{
    let zero = N::default();
    let one = m / m;

    let (gcd, y_inv, _) = ext_euclid(y, m);

    if gcd != one {panic!();}

    let y_inv = if y_inv < zero {y_inv + m} else {y_inv};

    (x * y_inv) % m
}

//#############################################################################

#[test]
fn test_ext_euclid() {
    assert_eq!(ext_euclid(48000, 44100).0, 300);
}

#[test]
fn test_mod_div() {
    let x = 3187689;
    let y = 7;
    let m = 23341823_i64;

    let z = mod_div(x, y, m);
    assert!(z > 0);
    assert!(z < m);
    assert_ne!(y * z, x);
    assert_eq!((y * z) % m, x);
}
