//! Helper `edge_midpoint`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn edge_midpoint(edge: &route_network::HighwayEdge) -> Option<(f64, f64)> {
    let coords = edge.geometry.0.as_slice();
    if coords.is_empty() {
        return None;
    }
    let idx = coords.len() / 2;
    let coord = coords[idx];
    Some((coord.y, coord.x))
}
