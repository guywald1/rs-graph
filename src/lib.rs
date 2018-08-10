#![cfg_attr(test, allow(dead_code))]

mod graph_impl;

pub mod graph {
    pub use graph_impl::{DiGraph};
}
