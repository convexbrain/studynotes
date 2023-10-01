use std::prelude::rust_2021::*;
use std::io::prelude::*;

macro_rules! dprintln {
    ( $($x:tt)* ) =>
    {
        #[cfg(debug_assertions)]
        {
            print!("[@{}:{}]", line!(), module_path!());
            println!($($x)*);
        }
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: u32 = spl.next().unwrap().parse().unwrap();
    let y: u32 = spl.next().unwrap().parse().unwrap();

    dprintln!("{} {}", n, y);

    let m10k = y / 10000;
    for i10k in 0..=m10k {
        let ry = y - i10k * 10000;

        let m5k = ry / 5000;
        for i5k in 0..=m5k {
            let rry = ry - i5k * 5000;

            let m1k = rry / 1000;

            dprintln!("{} {} {}", i10k, i5k, m1k);

            if i10k + i5k + m1k == n {
                println!("{} {} {}", i10k, i5k, m1k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
