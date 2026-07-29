//! Helper `tier_candidate_stitch_group_id`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_candidate_stitch_group_id(
    tier: &str,
    region_id: &str,
    route: &str,
    bundle_scope: &str,
) -> String {
    let identity = if bundle_scope.trim().is_empty() {
        format!("candidate-stitch|{tier}|{region_id}|{route}")
    } else {
        format!("candidate-stitch|{tier}|{region_id}|{route}|{bundle_scope}")
    };
    format!("US.HWYSTITCH.{:016X}", stable_segment_hash(&identity))
}

