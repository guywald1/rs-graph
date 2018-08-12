mod adj_list;

use self::adj_list::{AdjList, Edges, Nodes};
use std::cmp::Eq;
use std::collections::{HashMap,HashSet};
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
    let mut tree_visited: HashMap<&N, bool> = HashMap::with_capacity(graph.size());
    for node in graph.nodes() {
        total_visited.insert(node);
        tree_visited.insert(node, false);
    }

    while let Some(n) = total_visited.iter().next().map(|&n| n) {
        for node in graph.nodes() {
            tree_visited.insert(node, false);
        }
        let mut to_visit: Vec<&N> = vec![&n];
        while !to_visit.is_empty() {
            let curr = to_visit.pop().unwrap();
            if *tree_visited.get(curr).unwrap() == true {
                return true;
            }
            let children = graph.adj_list.list.get(curr).unwrap();
            to_visit.splice(0..0, children);
            tree_visited.insert(curr, true);
        }
        total_visited.remove(n);
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

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
        let di_graph = DiGraph::from(&[1,2,3,4], &[(1,2)]);
        assert_eq!(false, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1,2,3,4], &[(1,2), (2,1)]);
        assert_eq!(true, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1,2,3,4], &[]);
        assert_eq!(false, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1,2,3,4], &[(1,2), (2,3), (3,1)]);
        assert_eq!(true, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1,2,3,4], &[(1,4), (2,3), (4,1)]);
        assert_eq!(true, contains_cycle(&di_graph));
        let di_graph = DiGraph::from(&[1,2,3,4], &[(1,4), (2,3), (4,2)]);
        assert_eq!(false, contains_cycle(&di_graph));
    }
}
