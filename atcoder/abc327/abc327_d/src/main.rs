use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::*,
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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();
    let m: usize = token.next().unwrap().parse().unwrap();

    let a: Vec<u32> = (0..m).map(|_| {
        token.next().unwrap().parse::<u32>().unwrap() - 1
    }).collect();
    let b: Vec<u32> = (0..m).map(|_| {
        token.next().unwrap().parse::<u32>().unwrap() - 1
    }).collect();

    debug!(n, m, a, b);

    let mut g = HashMap::new();
    let mut q = VecDeque::new();

    let mut first = true;
    for a_b in a.iter().zip(b.iter()) {
        //debug!(a_b);
        let a = *a_b.0;
        let b = *a_b.1;

        if a == b {
            println!("No");
            return;
        }

        g.entry(a).or_insert((if first {Some(false)} else {None}, HashSet::new()));
        g.entry(b).or_insert((if first {Some(true)} else {None}, HashSet::new()));

        first = false;

        g.get_mut(&a).unwrap().1.insert(b);
        g.get_mut(&b).unwrap().1.insert(a);

        if let Some(pol) = g.get(&a).unwrap().0 {
            for ch in g.get(&a).unwrap().1.iter() {
                q.push_back((*ch, !pol));
            }
        }
        if let Some(pol) = g.get(&b).unwrap().0 {
            for ch in g.get(&b).unwrap().1.iter() {
                q.push_back((*ch, !pol));
            }
        }

        while let Some((ver, pol)) = q.pop_front() {
            if let Some(vpol) = g.get(&ver).unwrap().0 {
                if vpol != pol {
                    println!("No");
                    return;
                }
            }
            else {
                g.get_mut(&ver).unwrap().0 = Some(pol);
                for ch in g.get(&ver).unwrap().1.iter() {
                    q.push_back((*ch, !pol));
                }
            }
        }
    }
    debug!(g);
    
    println!("Yes");
}
