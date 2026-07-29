//! Helper `push_unique_neighbor`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn push_unique_neighbor(neighbors: &mut Vec<usize>, neighbor: usize) {
    if !neighbors.contains(&neighbor) {
        neighbors.push(neighbor);
        neighbors.sort_unstable();
    }
}

