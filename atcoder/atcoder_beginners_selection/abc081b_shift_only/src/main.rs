use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: usize = spl.next().unwrap().parse().unwrap();
    let mut min_sh = u32::MAX;

    for _ in 0.. n {
        let v: u32 = spl.next().unwrap().parse().unwrap();

        let mut vv = v;
        let mut sh = 0;

        while (vv & 1 == 0) && (vv > 0) {
            sh += 1;
            vv >>= 1;
        }

        if min_sh > sh {
            min_sh = sh;
        }

        //println!("{} {} {}", v, sh, min_sh);
    }

    println!("{}", min_sh);
}
