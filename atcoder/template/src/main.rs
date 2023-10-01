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

    let a = spl.next().unwrap();
    let b = spl.next().unwrap();
    let c = spl.next().unwrap();
    let d: u32 = spl.next().unwrap().parse().unwrap();

    dprintln!("{} {} {} {}", a, b, c, d);
}
