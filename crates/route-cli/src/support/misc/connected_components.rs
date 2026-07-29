//! Helper `connected_components`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn connected_components(adjacency: &[Vec<usize>]) -> (Vec<usize>, usize) {
    let mut component_ids = vec![usize::MAX; adjacency.len()];
    let mut component_count = 0usize;
    for start in 0..adjacency.len() {
        if component_ids[start] != usize::MAX {
            continue;
        }
        let mut queue = std::collections::VecDeque::from([start]);
        component_ids[start] = component_count;
        while let Some(node) = queue.pop_front() {
            for &neighbor in &adjacency[node] {
                if component_ids[neighbor] == usize::MAX {
                    component_ids[neighbor] = component_count;
                    queue.push_back(neighbor);
                }
            }
        }
        component_count += 1;
    }
    (component_ids, component_count)
}

