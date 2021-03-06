mod adj_list;

use self::adj_list::{AdjList, Edges, Nodes};
use std::cmp::Eq;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub struct DiGraph<N: Debug + Hash + Eq + Clone> {
    adj_list: AdjList<N>,
}

impl<'a, N: 'a + Debug + Hash + Eq + Clone + Copy> DiGraph<N> {
    pub fn new() -> DiGraph<N> {
        DiGraph {
            adj_list: AdjList::new(),
        }
    }

    pub fn from(nodes: &[N], edges: &[(N, N)]) -> DiGraph<N> {
        DiGraph {
            adj_list: AdjList::from(nodes, edges),
        }
    }

    pub fn nodes(&'a self) -> Nodes<N> {
        self.adj_list.nodes()
    }

    pub fn edges(&'a self) -> Edges<N> {
        self.adj_list.edges()
    }

    pub fn size(&self) -> usize {
        self.adj_list.size()
    }
}

pub fn contains_cycle<N>(graph: &DiGraph<N>) -> bool
where
    N: Debug + Hash + Eq + Clone + Copy,
{
    let mut total_visited: HashSet<&N> = HashSet::with_capacity(graph.size());
    for node in graph.nodes() {
        total_visited.insert(node);
    }
    let mut tree_visited: HashSet<&N> = HashSet::with_capacity(graph.size());

    while let Some(n) = total_visited.iter().next().map(|&n| n) {
        tree_visited.drain();
        let mut to_visit: Vec<&N> = vec![&n];
        while !to_visit.is_empty() {
            let curr = to_visit.pop().unwrap();
            if tree_visited.contains(curr) {
                return true;
            }
            let children = graph.adj_list.list.get(curr).unwrap();
            to_visit.splice(0..0, children);
            tree_visited.insert(curr);
        }
        total_visited.remove(n);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn nodes_edges_test() {
        let di_graph = DiGraph::from(&[1, 2, 3, 4], &[(1, 2), (2, 3), (3, 1)]);
        assert_eq!(4, di_graph.nodes().count());
        assert_eq!(3, di_graph.edges().count());
        let di_graph: DiGraph<usize> = DiGraph::new();
        assert_eq!(0, di_graph.nodes().count());
        assert_eq!(0, di_graph.edges().count());
    }

    #[test]
    fn size() {
        let di_graph = DiGraph::from(&[1, 2, 3, 4], &[(1, 2), (2, 3), (3, 1)]);
        assert_eq!(4, di_graph.size());
    }

    #[test]
    fn detect_cycle() {
        let di_graph = DiGraph::from(&[1, 2, 3, 4], &[(1, 2)]);
        assert_eq!(false, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1, 2, 3, 4], &[(1, 2), (2, 1)]);
        assert_eq!(true, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1, 2, 3, 4], &[]);
        assert_eq!(false, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1, 2, 3, 4], &[(1, 2), (2, 3), (3, 1)]);
        assert_eq!(true, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1, 2, 3, 4], &[(1, 4), (2, 3), (4, 1)]);
        assert_eq!(true, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1, 2, 3, 4], &[(1, 4), (2, 3), (4, 2)]);
        assert_eq!(false, contains_cycle(&di_graph));
        let nodes: Vec<usize> = (1..1_000).collect();
        let mut edges = vec![(1, 2), (2, 300), (300, 401), (401, 502), (502, 1)];
        edges.append(
            &mut (1..10)
                .collect::<Vec<usize>>()
                .iter()
                .map(|&i| (i, 2 * i))
                .collect::<Vec<(usize, usize)>>(),
        );
        let di_graph: DiGraph<usize> = DiGraph::from(&nodes, &edges);
        assert_eq!(true, contains_cycle(&di_graph));
    }
}
