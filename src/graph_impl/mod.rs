mod adj_list;

use self::adj_list::{AdjList, Nodes, Edge, Edges};
use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;

pub struct DiGraph<N: Debug + Hash + Eq + Clone> {
    adj_list: AdjList<N>,
}

impl<'a, N: 'a + Debug + Hash + Eq + Clone> DiGraph<N> {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nodes_edges_test() {
        let di_graph = DiGraph::from(&[1,2,3,4], &[(1,2), (2,3), (3,1)]);
        assert_eq!(4, di_graph.nodes().count());
        assert_eq!(3, di_graph.edges().count());
        let di_graph : DiGraph<usize> = DiGraph::new();
        assert_eq!(0, di_graph.nodes().count());
        assert_eq!(0, di_graph.edges().count());
    }
}
