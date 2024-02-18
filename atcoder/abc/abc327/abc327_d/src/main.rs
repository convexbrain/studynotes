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
    Visiting,
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
        unvisited: Option<BTreeSet<usize>>,
        bfs: bool,
        mut func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        let n = self.node_values.len();
        let mut unvis = unvisited.unwrap();//_or((0..n).collect());
        let mut que = VecDeque::new();

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        que.push_front((first_node, first_weight, first_travel));

        while let Some((u, w, t)) = que.pop_front() {
            if unvis.contains(&u) {
                unvis.remove(&u);

                let nt = func(NodeSt::Visiting, &mut self.node_values[u], w, t);

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
        unvisited: Option<BTreeSet<usize>>,
        func: F) -> BTreeSet<usize>
    where F: FnMut(NodeSt, &mut V, W, T) -> T, T: Copy {

        self._traverse(first_node, first_weight, first_travel, unvisited, false, func)
    }
}

//#############################################################################

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();
    let m: usize = token.next().unwrap().parse().unwrap();

    let a: Vec<usize> = (0..m).map(|_| token.next().unwrap().parse().unwrap()).collect();
    let b: Vec<usize> = (0..m).map(|_| token.next().unwrap().parse().unwrap()).collect();

    let mut g = Graph::new_nodes(n, 0, true);

    for i in 0..m {
        g.add_edge(a[i] - 1, b[i] - 1, ());
    }

    let mut unvis: BTreeSet<usize> = (0..n).collect();
    while !unvis.is_empty() {
        let mut okay = true;

        unvis = g.dfs(None, (), 0, Some(unvis),
            |st, v, _w, t| {
                match st {
                    NodeSt::Visiting => {
                        *v = t;
                        1 - t
                    },
                    NodeSt::Returned => {panic!();},
                    NodeSt::Visited => {
                        if *v != t {
                            okay = false;
                        }
                        t
                    },
                }
            });
        
        if !okay {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
