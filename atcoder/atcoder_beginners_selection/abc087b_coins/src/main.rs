use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let a: u32 = spl.next().unwrap().parse().unwrap();
    let b: u32 = spl.next().unwrap().parse().unwrap();
    let c: u32 = spl.next().unwrap().parse().unwrap();
    let x: u32 = spl.next().unwrap().parse().unwrap();

    eprintln!("{} {} {} {}", a, b, c, x);

    let mut cnt = 0;
    
    let xa = x;
    let ma = xa / 500;
    let ma = if ma <= a {ma} else {a};

    for ia in 0..= ma {
        let xb = xa - ia * 500;
        let mb = xb / 100;
        let mb = if mb <= b {mb} else {b};

        for ib in 0..= mb {
            let xc = xb - ib * 100;
            let mc = xc / 50;
            let mc = if mc <= c {mc} else {c};

            eprintln!("{} {} {}", ma, mb, mc);

            for ic in 0..= mc {
                if ia * 500 + ib * 100 + ic * 50 == x {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}
