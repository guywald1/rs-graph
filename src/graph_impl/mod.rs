mod adj_list;

use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;
use self::adj_list::AdjList;

pub struct DiGraph<N: Debug + Hash + Eq + Clone> {
    adj_list: AdjList<N>
}

impl<N: Debug + Hash + Eq + Clone> DiGraph<N> {
    pub fn new() -> DiGraph<N> {
        DiGraph {
            adj_list: AdjList::new()
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::DiGraph;
// }
