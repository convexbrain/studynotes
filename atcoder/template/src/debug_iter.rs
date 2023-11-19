//#############################################################################

#[cfg(not(debug_assertions))]
macro_rules! debug_iter {
    ( $($x:tt)* ) => {};
}

#[cfg(debug_assertions)]
macro_rules! debug_iter {
    ($it:expr, $num:expr) => {
        {
            eprint!("[@{}] {} = [", line!(), stringify!($it));
            let mut tmp_it = $it;
            let tmp_num = $num as usize;
            let mut tmp_cnt = 0_usize;
            while let Some(item) = tmp_it.next() {
                if tmp_cnt >= tmp_num { break; }
                else if tmp_cnt > 0 { eprint!(", "); }
                eprint!("{:?}", item);
                tmp_cnt += 1;
            }
            eprintln!("]");
        }
    };
    ($it:expr) => {
        debug_iter!($it, usize::MAX);
    };
}

//#############################################################################

#[test]
fn test_debug_iter() {
    // cargo test -- --nocapture
    let s = module_path!();
    debug_iter!(s.chars(), 8);
    debug_iter!(s.chars());
}
