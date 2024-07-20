use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::{*, Bound::*}, cmp::*,
    rc::*, cell::*,
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

struct Tokens<'a>(std::str::SplitWhitespace<'a>);

#[allow(dead_code)]
impl<'a> Tokens<'a> {
    fn new(buf: &'a mut String) -> Self {
        std::io::stdin().read_to_string(buf).unwrap();
        Tokens(buf.split_whitespace())
    }
    fn release(self) -> std::str::SplitWhitespace<'a> {
        self.0
    }
    fn next_string(&mut self) -> String {
        self.0.next().unwrap().to_string()
    }
    fn next_bytes(&mut self) -> Vec<u8> {
        self.0.next().unwrap().as_bytes().to_vec()
    }
    fn next<T>(&mut self) -> T
    where T: std::str::FromStr, T::Err: std::fmt::Debug {
        self.0.next().unwrap().parse().unwrap()
    }
    fn collect<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<T> {
        (0..n).map(|_| self.next()).collect()
    }
    fn collect_index<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<(usize, T)> {
        (0..n).map(|i| (i, self.next())).collect()
    }
}

//#############################################################################

#[derive(Debug, Clone)]
struct Edge<W> {
    weight: W,
    from: usize,
    to: usize,
}

impl<W: Copy> Edge<W> {
    fn node_from(&self, u: usize) -> (usize, W) {
        let nu = if u != self.to {
            self.to
        }
        else {
            self.from
        };

        (nu, self.weight)
    }
}

#[derive(Debug, Clone)]
enum NodeSt {
    Unvisited,
    Returned,
    Visited,
}

#[derive(Debug, Clone)]
struct Graph<V, W> {
    node_values: Vec<V>,
    node_edges: Vec<BTreeSet<usize>>,
    edges: Vec<Edge<W>>,
    undir: bool,
}

impl<V, W: Copy> Graph<V, W> {
    fn new(undir: bool) -> Self {
        Graph {
            node_values: Vec::new(),
            node_edges: Vec::new(),
            edges: Vec::new(),
            undir,
        }
    }

    fn new_nodes(n: usize, value: V, undir: bool) -> Self
    where V: Clone {
        Graph {
            node_values: vec![value; n],
            node_edges: vec![BTreeSet::new(); n],
            edges: Vec::new(),
            undir,
        }
    }

    fn add_node(&mut self, value: V) {
        self.node_values.push(value);
        self.node_edges.push(BTreeSet::new());
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        let edge = Edge {
            weight,
            from,
            to,
        };

        let edge_idx = self.edges.len();

        self.edges.push(edge);

        self.node_edges[from].insert(edge_idx);
        if self.undir {
            self.node_edges[to].insert(edge_idx);
        }
    }

    fn node_values(&self) -> &[V] {
        &self.node_values
    }

    fn _traverse<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        mut unvis: BTreeSet<usize>,
        bfs: bool,
        mut func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        let mut que = VecDeque::new();

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        que.push_front((first_node, first_weight, first_travel));

        while let Some((u, w, t)) = que.pop_front() {
            if unvis.contains(&u) {
                unvis.remove(&u);

                let nt = func(NodeSt::Unvisited, &mut self.node_values[u], w, t);

                for &e in self.node_edges[u].iter() {
                    let (nu, nw) = self.edges[e].node_from(u);

                    if bfs {
                        que.push_back((nu, nw, nt));
                    }
                    else {
                        que.push_front((nu, nw, nt));
                    }
                }
            }
            else {
                func(NodeSt::Visited, &mut self.node_values[u], w, t);
            }
        }

        unvis
    }

    fn dfs<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        unvis: BTreeSet<usize>,
        func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        self._traverse(first_node, first_weight, first_travel, unvis, false, func)
    }
    
    fn bfs<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        unvis: BTreeSet<usize>,
        func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        self._traverse(first_node, first_weight, first_travel, unvis, true, func)
    }
    
    fn dijkstra<U>(&mut self,
        first_node: Option<usize>,
        mut unvis: BTreeSet<usize>,
        mut update: U) -> BTreeSet<usize>
    where U: FnMut(&mut V, W, usize) -> bool, W: Ord + Add<Output=W> + Default {

        let mut que = BinaryHeap::new();

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        que.push((Reverse(W::default()), first_node, first_node));

        while let Some((ws, u, prev_u)) = que.pop() {
            unvis.remove(&u);

            if update(&mut self.node_values[u], ws.0, prev_u) {

                for &e in self.node_edges[u].iter() {
                    let (nu, nw) = self.edges[e].node_from(u);
                    let nws = ws.0 + nw;

                    que.push((Reverse(nws), nu, u));
                }
            }
        }

        unvis
    }
    
    fn _dfs_rec<T, F>(
        node_values: &mut[V], node_edges: &[BTreeSet<usize>], edges: &[Edge<W>],
        u: usize, w: W, t: T,
        unvis: &mut BTreeSet<usize>,
        func: &mut F)
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        if unvis.contains(&u) {
            unvis.remove(&u);

            let nt = func(NodeSt::Unvisited, &mut node_values[u], w, t);

            for &e in node_edges[u].iter() {
                let (nu, nw) = edges[e].node_from(u);

                Self::_dfs_rec(node_values, node_edges, edges, nu, nw, nt, unvis, func);
            }

            func(NodeSt::Returned, &mut node_values[u], w, t);
        }
        else {
            func(NodeSt::Visited, &mut node_values[u], w, t);
        }
    }

    fn dfs_rec<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        mut unvis: BTreeSet<usize>,
        mut func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        Self::_dfs_rec(
            &mut self.node_values, &self.node_edges, &self.edges,
            first_node, first_weight, first_travel,
            &mut unvis,
            &mut func);

        unvis
    }

    fn dijkstra2(&mut self, vs: &[V])
    where W: Ord + Add<Output=W> + Default, V: Ord + Copy + Default + Add<V, Output=V> + Add<W, Output=V> {

        let mut que = BinaryHeap::new();

        let first_node = 0;

        que.push((Reverse(vs[first_node]), first_node, first_node, W::default()));

        while let Some((new_v, u, prev_u, w)) = que.pop() {
            debug!(prev_u, u);
            
            if self.node_values[u] > new_v.0 {
                self.node_values[u] = new_v.0;

                for &e in self.node_edges[u].iter() {
                    let (nu, nw) = self.edges[e].node_from(u);
                    let nv = new_v.0 + nw + vs[nu];

                    que.push((Reverse(nv), nu, u, nw));
                }
            }
        }
    }
}

//#############################################################################

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let m: usize = tokens.next();
    let a: Vec<u64> = tokens.collect(n);
    let uvb : Vec<(usize, usize, u64)> = (0..m).map(|i| (tokens.next(), tokens.next(), tokens.next())).collect();

    let mut g = Graph::new_nodes(n, u64::MAX, true);
    for (u, v, b) in uvb.iter() {
        g.add_edge(*u - 1, *v - 1, *b);
    }

    g.dijkstra2(&a);
    
    let gg = g.node_values();
    debug!(gg);

    for i in 1..n {
        print!("{} ", gg[i]);
    }
    println!();
}
