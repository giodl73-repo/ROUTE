//! Helper `shared_segment_pair_id`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn shared_segment_pair_id(route: &str, overlap_route: &str) -> String {
    let mut routes = [route_display_key(route), route_display_key(overlap_route)];
    routes.sort();
    routes.join("-")
}
