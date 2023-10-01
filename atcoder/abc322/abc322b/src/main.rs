use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: u32 = spl.next().unwrap().parse().unwrap();
    let m: u32 = spl.next().unwrap().parse().unwrap();
    let s = spl.next().unwrap();
    let t = spl.next().unwrap();

    eprintln!("{} {} {} {}", n, m, s, t);

    let mut start = false;
    let mut end = false;

    if let Some(p) = t.find(s) {
        if p == 0 {
            start = true;
        }
    }

    if let Some(p) = t.rfind(s) {
        if p == t.len() - s.len() {
            end = true;
        }
    }

    let v = if start && end {0} else {
        if start && !end {1} else {
            if !start && end {2} else {
                3
            }
        }
    };

    println!("{}", v);
}
