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

fn rec0(
    graph: &mut Vec<(u32, HashSet<usize>, HashSet<usize>)>,
    vis: &mut HashSet<usize>,
    depth: &mut u32,
    v: usize,
)
{
    if !vis.contains(&v) {
        vis.insert(v);

        let chs: Vec<usize> = graph[v].1.iter().map(|x| *x).collect();
        for ch in chs.iter() {
            rec0(graph, vis, depth, *ch);
        }
        *depth += 1;
        graph[v].0 = *depth;
    }
}

fn rec1(
    graph: &mut Vec<(u32, HashSet<usize>, HashSet<usize>)>,
    vis: &mut HashSet<usize>,
    depth: &mut u32,
    v: usize,
)
{
    if !vis.contains(&v) {
        vis.insert(v);

        let pas: Vec<usize> = graph[v].2.iter().map(|x| *x).collect();
        for pa in pas.iter() {
            rec1(graph, vis, depth, *pa);
        }
        graph[v].0 = *depth;
        *depth += 1;
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();
    let m: usize = token.next().unwrap().parse().unwrap();

    let mut graph = vec![(0_u32, HashSet::<usize>::new(), HashSet::<usize>::new()); n];

    for _ in 0..m {
        let a: usize = token.next().unwrap().parse().unwrap();
        let b: usize = token.next().unwrap().parse().unwrap();
        let a = a - 1;
        let b = b - 1;

        graph[a].1.insert(b);
        graph[b].2.insert(a);
    }

    let mut vis = HashSet::new();
    let mut depth = 0;

    for v in 0..n {
        rec0(&mut graph, &mut vis, &mut depth, v);
    }

    debug!(graph);
    
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|x| graph[*x].0);

    debug!(order);

    vis.clear();

    let mut pairs = 0;
    for v in order.iter().rev() {
        let mut depth = 0;
        rec1(&mut graph, &mut vis, &mut depth, *v);

        debug!(depth);

        if depth > 1 {
            let depth = depth as u64;
            pairs += depth * (depth - 1) / 2;
        }
    }
    
    println!("{pairs}");
}
