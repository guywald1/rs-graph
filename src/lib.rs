#![cfg_attr(test, allow(dead_code))]
#![feature(test)]

extern crate test;

mod graph_impl;

use graph_impl::{contains_cycle, DiGraph};

pub mod graph {
    pub use graph_impl::{contains_cycle, DiGraph};
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_contains_cycles(b: &mut Bencher) {
        for _ in 0..10 {
            b.iter(|| {
                let nodes: Vec<usize> = (1..10_000).collect();
                let mut edges = vec![(1, 2), (2, 300), (300, 401), (401, 502), (502, 1)];
                edges.append(
                    &mut (1..10)
                        .collect::<Vec<usize>>()
                        .iter()
                        .map(|&i| (i, 2 * i))
                        .collect::<Vec<(usize, usize)>>(),
                );
                let di_graph: DiGraph<usize> = DiGraph::from(&nodes, &edges);
                contains_cycle(&di_graph)
            })
        }
    }
}
