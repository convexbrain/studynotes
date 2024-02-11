use std::{ops::*, collections::*, cmp::*};

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

    fn traverse<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        unvisited: Option<BTreeSet<usize>>,
        bfs: bool,
        mut func: F) -> BTreeSet<usize>
    where F: FnMut(usize, &mut V, W, T) -> T, T: Copy {

        let n = self.node_values.len();
        let mut unvis = unvisited.unwrap_or((0..n).collect());
        let mut que = VecDeque::new();

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        que.push_front((first_node, first_weight, first_travel));

        while let Some((u, w, t)) = que.pop_front() {
            if unvis.contains(&u) {
                unvis.remove(&u);

                let nt = func(u, &mut self.node_values[u], w, t);

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
        }

        unvis
    }
    
    fn dijkstra<U>(&mut self,
        first_node: Option<usize>,
        unvisited: Option<BTreeSet<usize>>,
        mut update: U) -> BTreeSet<usize>
    where U: FnMut(usize, usize, &mut V, W) -> bool, W: Ord + Add<Output=W> + Default {

        let n = self.node_values.len();
        let mut unvis = unvisited.unwrap_or((0..n).collect());
        let mut que = BinaryHeap::new();

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        que.push((Reverse(W::default()), first_node, first_node));

        while let Some((ws, u, prev_u)) = que.pop() {
            unvis.remove(&u);

            if update(u, prev_u, &mut self.node_values[u], ws.0) {

                for &e in self.node_edges[u].iter() {
                    let (nu, nw) = self.edges[e].node_from(u);
                    let nws = ws.0 + nw;

                    que.push((Reverse(nws), nu, u));
                }
            }
        }

        unvis
    }
    
    fn dfs_rec<T, F1, F2>(
        node_values: &mut[V], node_edges: &[BTreeSet<usize>], edges: &[Edge<W>],
        u: usize, w: W, t: T,
        unvis: &mut BTreeSet<usize>,
        func_pre: &mut F1, func_post: &mut F2)
    where F1: FnMut(usize, &mut V, W, T) -> T, F2: FnMut(usize, &mut V), T: Copy {

        if unvis.contains(&u) {
            unvis.remove(&u);

            let nt = func_pre(u, &mut node_values[u], w, t);

            for &e in node_edges[u].iter() {
                let (nu, nw) = edges[e].node_from(u);

                Self::dfs_rec(node_values, node_edges, edges, nu, nw, nt, unvis, func_pre, func_post);
            }

            func_post(u, &mut node_values[u]);
        }
    }

    fn dfs<T, F1, F2>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        unvisited: Option<BTreeSet<usize>>,
        func_pre: &mut F1, func_post: &mut F2) -> BTreeSet<usize>
    where F1: FnMut(usize, &mut V, W, T) -> T, F2: FnMut(usize, &mut V), T: Copy {

        let n = self.node_values.len();
        let mut unvis = unvisited.unwrap_or((0..n).collect());

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        Self::dfs_rec(
            &mut self.node_values, &self.node_edges, &self.edges,
            first_node, first_weight, first_travel,
            &mut unvis,
            func_pre, func_post);

        unvis
    }
}

//#############################################################################
