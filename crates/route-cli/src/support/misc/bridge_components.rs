//! Helper `bridge_components`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn bridge_components(
    adjacency: &mut [Vec<usize>],
    component_ids: &[usize],
    component_count: usize,
) {
    let mut representatives = vec![None; component_count];
    for (node, &component) in component_ids.iter().enumerate() {
        representatives[component].get_or_insert(node);
    }
    for pair in representatives.windows(2) {
        if let [Some(a), Some(b)] = pair {
            push_unique_neighbor(&mut adjacency[*a], *b);
            push_unique_neighbor(&mut adjacency[*b], *a);
        }
    }
}

