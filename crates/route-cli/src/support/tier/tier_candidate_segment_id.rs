//! Helper `tier_candidate_segment_id`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_candidate_segment_id(edge: &route_network::HighwayEdge) -> String {
    let first = edge.geometry.0.first().copied();
    let last = edge.geometry.0.last().copied();
    let geometry_key = match (first, last) {
        (Some(first), Some(last)) => {
            format!("{:.5},{:.5}->{:.5},{:.5}", first.x, first.y, last.x, last.y)
        }
        _ => "missing-geometry".to_string(),
    };
    format!(
        "US.HWYSEG.{:016X}",
        stable_segment_hash(&format!(
            "edge|{}|{}|{}|{:.3}",
            edge.route_id, edge.state, geometry_key, edge.length_miles
        ))
    )
}
