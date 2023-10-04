use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;

macro_rules! dprintln {
    ( $($x:tt)* ) =>
    {
        #[cfg(debug_assertions)]
        {
            print!("[{}]", line!());
            println!($($x)*);
        }
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let s = spl.next().unwrap();

    dprintln!("{}", s);

    let strs = ["dream", "dreamer", "erase", "eraser"];

    let mut stage_sel = BTreeMap::<usize, (usize, usize)>::new();
    let mut stage = 0usize;
    let mut cnt_sel = 0usize;
    let mut offset = 0usize;
    loop {
        dprintln!("{}", offset);

        let so = s.split_at(offset).1;
        if so.len() == 0 {
            println!("YES");
            return;
        }

        if so.starts_with(strs[cnt_sel]) {
            stage_sel.insert(stage, (offset, cnt_sel));

            offset += strs[cnt_sel].len();
            stage += 1;
            cnt_sel = 0;
        }
        else {
            if cnt_sel < strs.len() - 1 {
                cnt_sel += 1;
            }
            else {
                loop {
                    if stage > 0 {
                        stage -= 1;
                        (offset, cnt_sel) = stage_sel[&stage];
                        if cnt_sel < strs.len() - 1 {
                            cnt_sel += 1;
                            break;
                        }
                    }
                    else {
                        println!("NO");
                        return;
                    }
                }
            }
        }

    }
}
