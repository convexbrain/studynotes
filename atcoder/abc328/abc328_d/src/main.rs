use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::*, cmp::*,
    rc::*, cell::*, ops::Bound::*,
};

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ( $($x:tt)* ) => {};
}

#[cfg(debug_assertions)]
macro_rules! debug {
    () => {
        eprintln!("[@{}]", line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            ref tmp => {
                eprintln!("[@{}] {} = {:?}",
                    line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(debug!($val)),+,)
    };
}

//#############################################################################

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let s = token.next().unwrap();
    let pat = "ABC";

    let mut q = VecDeque::new();

    let mut st = 0_usize;
    for &c in s.as_bytes() {
        //debug!(c, st);
        //debug!(c, b'A' + st as u8);
        if c == b'A' + st as u8 {
            //debug!(c);
            st += 1;
            if st > 2 {
                if let Some(pst) = q.pop_front() {
                    st = pst;
                }
                else {
                    st = 0;
                }
            }
        }
        else {
            if c == b'A' {
                q.push_front(st);
                st = 1;
            }
            else {
                //debug!(c, st);
                while let Some(pst) = q.pop_back() {
                    //debug!(pst);
                    let ps = pat.split_at(pst).0;
                    print!("{ps}");
                }
                let ps = pat.split_at(st).0;
                print!("{ps}");
                let ch = String::from_utf8(vec![c]).unwrap();
                print!("{ch}");

                st = 0;
            }
        }
    }

    while let Some(pst) = q.pop_back() {
        let ps = pat.split_at(pst).0;
        print!("{ps}");
    }
    let ps = pat.split_at(st).0;
    print!("{ps}");
    println!();
}
