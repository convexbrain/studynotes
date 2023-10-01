use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: usize = spl.next().unwrap().parse().unwrap();
    let q: usize = spl.next().unwrap().parse().unwrap();
    let mut s: String = spl.next().unwrap().into();

    //eprintln!("#{} {} {}", n, q, s);

    for _ in 0.. q {
        let c: usize = spl.next().unwrap().parse().unwrap();
        let l: usize = spl.next().unwrap().parse().unwrap();
        let r: usize = spl.next().unwrap().parse().unwrap();

        //eprintln!("#{} {} {}", c, l, r);

        if c == 1 {
            //eprintln!("#{}", s);

            let ptr = s.as_mut_ptr();
            for i in (l - 1)..= (r - 1) {
                unsafe {
                    let val = *ptr.offset(i as isize);
                    *ptr.offset(i as isize) = if val == '1' as u8 {'0'} else {'1'} as u8;
                }
            }
            /*
            let (s0, s1) = s.split_at_mut(l - 1);
            let (sm, s2) = s1.split_at_mut(r - l + 1);
            //eprintln!("#{}", sm);

            let rsm = sm.replace("0", "o").replace("1", "0").replace("o", "1");
            //eprintln!("#{}", rsm);

            let mut ns = String::from(s0);
            ns.push_str(&rsm);
            ns.push_str(s2);
            //eprintln!("#{}", ns);

            s = ns;
            //eprintln!("#{}", s);
             */
        }
        else {
            let t = s.split_at(l - 1).1.split_at(r - l + 1).0;
            //eprintln!("#{}", t);

            let mut max_len = 0;
            for ones in t.split("0") {
                //eprintln!("#{} {}", ones, ones.len());
                if ones.len() > max_len {
                    max_len = ones.len();
                }
            }
            println!("{}", max_len);
        }
    }
}
