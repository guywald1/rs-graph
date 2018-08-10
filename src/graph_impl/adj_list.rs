use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map::Keys;

pub struct AdjList<N : Debug + Hash + Clone + Eq> {
    list: HashMap<N, Vec<N>>
}

pub struct Nodes<'a, N : 'a> {
    inner: Keys<'a, N, Vec<N>>
}

impl<'a, N> Iterator for Nodes<'a, N> where N : 'a {
    type Item = &'a N;
    fn next(&mut self) -> Option<&'a N> {
        self.inner.next()
    }
}

#[derive(Debug, PartialEq)]
pub struct Edge<N>(N, N);

pub struct Edges<'a, N : 'a + Debug + Hash + Clone + Eq> {
    keys_iter: Keys<'a, N, Vec<N>>,
    curr_key: Option<&'a N>,
    curr_index: usize,
    list: &'a HashMap<N, Vec<N>>
}

impl<'a, N: 'a + Debug + Hash + Clone + Eq> Iterator for Edges<'a, N> {
    type Item = Edge<&'a N>;

    fn next (&mut self) -> Option<Edge<&'a N>> {
        if self.curr_key.is_none() {
            return None;
        }
        let curr_key = self.curr_key.unwrap();
        let vec = self.list.get(curr_key).unwrap();
        if self.curr_index == vec.len() {
            let next_key = self.keys_iter.next();
            if next_key.is_none() {
                return None;
            }
            self.curr_index = 0;
            self.curr_key = next_key;
        }
        let other_key = &self.list.get(&self.curr_key.unwrap()).unwrap()[self.curr_index];
        self.curr_index += 1;
        Some(Edge(self.curr_key.unwrap(), other_key))
    }
}

impl<N : Debug + Hash + Clone + Eq> AdjList<N> {
    pub fn new() -> AdjList<N> {
        AdjList {
            list: HashMap::new()
        }
    }
    pub fn from(nodes: &[N], edges: &[(N, N)]) -> AdjList<N> {
        let mut list: HashMap<N, Vec<N>> = HashMap::new();
        list.reserve(nodes.len());
        for n in nodes {
            list.insert(n.clone(), Vec::new());
        }
        for (k, v) in edges {
            list
                .get_mut(k)
                .unwrap()
                .push(v.clone());
        }
        AdjList { list }
    }
    pub fn nodes(&self) -> Nodes<N> {
        Nodes { inner: self.list.keys() }
    }
    pub fn edges<'a> (&'a self) -> Edges<N> {
        let mut keys_iter = self.list.keys();
        let curr_key = keys_iter.next();
        Edges { list: &self.list, keys_iter, curr_key, curr_index: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_from_empty() {
        let list: AdjList<usize> = AdjList::new();
        assert_eq!(list.nodes().count(), 0);
        let nodes = list.nodes().collect::<Vec<_>>();
        assert_eq!(nodes.len(), 0);
        let edges = list.edges().collect::<Vec<_>>();
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn nodes_test() {
        let list: AdjList<usize> = AdjList::new();
        assert_eq!(list.nodes().count(), 0);
        let list = AdjList::from(&[1,2,3,4], &[(1,2), (2,3), (3,1), (4,2)]);
        let mut nodes = list.nodes().collect::<Vec<_>>();
        nodes.sort();
        assert_eq!(4, nodes.len());
        for n in 0..4 {
            assert_eq!(n+1, *nodes[n]);
        }
    }

    #[test]
    fn edges_test() {
        let mut edges = [(1,2), (2,3), (3,1), (4,2)];
        let list = AdjList::from(&[1,2,3,4], &edges);
        edges.sort();
        let mut edges_from_list = list.edges().map(|Edge(n,m)| (*n,*m)).collect::<Vec<_>>();
        edges_from_list.sort();
        assert_eq!(edges_from_list, edges);
    }
}
