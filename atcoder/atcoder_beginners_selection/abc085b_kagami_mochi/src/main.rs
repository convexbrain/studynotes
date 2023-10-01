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

    let n: usize = spl.next().unwrap().parse().unwrap();
    dprintln!("{}", n);

    let mut d = vec![0; n];
    for i in 0.. n {
        d[i] = spl.next().unwrap().parse().unwrap();
    }
    dprintln!("{:?}", d);

    d.sort();
    dprintln!("{:?}", d);

    let r = d.iter().fold((-1, 0), |(prev, cnt), x| if prev == *x {(*x, cnt)} else {(*x, cnt + 1)});
    println!("{}", r.1);
}
