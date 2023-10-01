use std::io;
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
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n = spl.next().unwrap().parse().unwrap();

    dprintln!("{}", n);

    let mut a = vec![0; n];

    for i in 0.. n {
        a[i] = spl.next().unwrap().parse().unwrap();
    }

    dprintln!("{:?}", a);

    a.sort();
    a.reverse();
    dprintln!("{:?}", a);
    let sum = a.iter().enumerate().fold((0, 0), |(sa, sb), (i, x)| if i & 1 == 0 {(sa + x, sb)} else {(sa, sb + x)});
    dprintln!("{:?}", sum);

    println!("{}", sum.0 - sum.1);
}
