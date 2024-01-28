use std::{ops::*, collections::*};

//#############################################################################

#[derive(Debug, Clone)]
struct Graph<T> {
    nodes: Vec<T>,
    edges: Vec<BTreeSet<usize>>,
}

impl<T> Graph<T> {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn new_n(n: usize, t: T) -> Self
    where T: Clone {
        Graph {
            nodes: vec![t; n],
            edges: vec![BTreeSet::new(); n],
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn add_node(&mut self, t: T) {
        self.nodes.push(t);
        self.edges.push(BTreeSet::new());
    }

    fn add_edge(&mut self, u: usize, v: usize, bi_dir: bool) {
        self.edges[u].insert(v);
        if bi_dir {
            self.edges[v].insert(u);
        }
    }

    fn as_slice(&self) -> &[T] {
        &self.nodes
    }

    fn nodes(self) -> Vec<T> {
        self.nodes
    }

    fn traverse<F>(&mut self, unvisited: Option<BTreeSet<usize>>, bfs: bool, mut func: F) -> BTreeSet<usize>
    where F: FnMut(usize, &mut T) {

        let mut unvis = unvisited.unwrap_or((0..self.len()).collect());
        let mut que = VecDeque::new();

        if let Some(&u) = unvis.first() {
            que.push_front(u);

            while let Some(v) = que.pop_front() {
                if unvis.contains(&v) {
                    unvis.remove(&v);
    
                    func(v, &mut self.nodes[v]);
    
                    for &cv in self.edges[v].iter() {
                        if bfs {
                            que.push_back(cv);
                        }
                        else {
                            que.push_front(cv);
                        }
                    }
                }
            }
        }

        unvis
    }

    fn dfs_rec<F1, F2>(
        nodes: &mut[T], edges: &[BTreeSet<usize>],
        unvisited: &mut BTreeSet<usize>, u: usize,
        func_pre: &mut F1, func_post: &mut F2)
    where F1: FnMut(usize, &mut T), F2: FnMut(usize, &mut T) {

        if unvisited.contains(&u) {
            unvisited.remove(&u);

            func_pre(u, &mut nodes[u]);

            for &cu in edges[u].iter() {
                Self::dfs_rec(nodes, edges, unvisited, cu, func_pre, func_post);
            }

            func_post(u, &mut nodes[u]);
        }
    }

    fn dfs<F1, F2>(&mut self,
        unvisited: Option<BTreeSet<usize>>,
        func_pre: &mut F1, func_post: &mut F2) -> BTreeSet<usize>
    where F1: FnMut(usize, &mut T), F2: FnMut(usize, &mut T) {

        let mut unvis = unvisited.unwrap_or((0..self.len()).collect());

        if let Some(&u) = unvis.first() {
            Self::dfs_rec(&mut self.nodes, &self.edges, &mut unvis, u, func_pre, func_post);
        }

        unvis
    }
}

impl<T> Index<usize> for Graph<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<T> IndexMut<usize> for Graph<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

//#############################################################################
