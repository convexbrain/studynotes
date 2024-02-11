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

    fn _traverse<T, F>(&mut self,
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

    fn dfs<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        unvisited: Option<BTreeSet<usize>>,
        func: F) -> BTreeSet<usize>
    where F: FnMut(usize, &mut V, W, T) -> T, T: Copy {

        self._traverse(first_node, first_weight, first_travel, unvisited, false, func)
    }
    
    fn bfs<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        unvisited: Option<BTreeSet<usize>>,
        func: F) -> BTreeSet<usize>
    where F: FnMut(usize, &mut V, W, T) -> T, T: Copy {

        self._traverse(first_node, first_weight, first_travel, unvisited, true, func)
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
    
    fn _dfs_rec<T, F>(
        node_values: &mut[V], node_edges: &[BTreeSet<usize>], edges: &[Edge<W>],
        u: usize, w: W, t: T,
        unvis: &mut BTreeSet<usize>,
        func_pre_post: &mut F)
    where F: FnMut(usize, &mut V, Option<(W, T)>) -> T, T: Copy {

        if unvis.contains(&u) {
            unvis.remove(&u);

            let nt = func_pre_post(u, &mut node_values[u], Some((w, t)));

            for &e in node_edges[u].iter() {
                let (nu, nw) = edges[e].node_from(u);

                Self::_dfs_rec(node_values, node_edges, edges, nu, nw, nt, unvis, func_pre_post);
            }

            func_pre_post(u, &mut node_values[u], None);
        }
    }

    fn dfs_rec<T, F>(&mut self,
        first_node: Option<usize>, first_weight: W, first_travel: T,
        unvisited: Option<BTreeSet<usize>>,
        mut func_pre_post: F) -> BTreeSet<usize>
    where F: FnMut(usize, &mut V, Option<(W, T)>) -> T, T: Copy {

        let n = self.node_values.len();
        let mut unvis = unvisited.unwrap_or((0..n).collect());

        let first_node = first_node.unwrap_or(*unvis.first().unwrap());

        Self::_dfs_rec(
            &mut self.node_values, &self.node_edges, &self.edges,
            first_node, first_weight, first_travel,
            &mut unvis,
            &mut func_pre_post);

        unvis
    }
}

//#############################################################################

#[test]
fn test_graph_dfs_bfs() {
    let mut g = Graph::new_nodes(7, 0, true);
    g.add_edge(0, 1, ());
    g.add_edge(0, 2, ());
    g.add_edge(1, 3, ());
    g.add_edge(1, 4, ());
    g.add_edge(2, 5, ());
    g.add_edge(2, 6, ());

    let mut cnt = 0;
    g.dfs(None, (), (), None,
        |_u, v, _w, _t| {
            *v = cnt;
            cnt += 1;
        });
    
    assert_eq!(g.node_values(), [0, 4, 1, 6, 5, 3, 2]);

    let mut cnt = 0;
    g.bfs(None, (), (), None,
        |_u, v, _w, _t| {
            *v = cnt;
            cnt += 1;
        });
    
    assert_eq!(g.node_values(), [0, 1, 2, 3, 4, 5, 6]);

    let mut cnt = 0;
    g.dfs_rec(None, (), (), None,
        |_u, v, wt| {
            if let Some((_w, _t)) = wt {
            }
            else {
                *v = cnt;
                cnt += 1;
            }
            ()
        },
    );
    
    assert_eq!(g.node_values(), [6, 2, 5, 0, 1, 3, 4]);
}

#[test]
fn test_graph_dfs_dijkstra() {
    let mut g = Graph::new_nodes(6, u16::MAX, true);
    g.add_edge(0, 1, 1);
    g.add_edge(0, 2, 2);
    g.add_edge(0, 3, 3);
    g.add_edge(0, 4, 4);
    g.add_edge(1, 5, 40);
    g.add_edge(2, 5, 30);
    g.add_edge(3, 5, 20);
    g.add_edge(4, 5, 10);

    g.dijkstra(Some(0), None,
        |_u, _p, v, w| {
            if *v > w {
                *v = w;
                true
            }
            else {
                false
            }
        });
    
    assert_eq!(g.node_values(), [0, 1, 2, 3, 4, 14]);
}
