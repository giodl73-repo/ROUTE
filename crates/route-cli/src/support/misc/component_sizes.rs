//! Helper `component_sizes`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn component_sizes(component_ids: &[usize], component_count: usize) -> Vec<usize> {
    let mut sizes = vec![0usize; component_count];
    for &component in component_ids {
        sizes[component] += 1;
    }
    sizes
}
