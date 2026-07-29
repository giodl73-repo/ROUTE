//! Helper `t3_segment_bundle_id`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_segment_bundle_id(zone_id: &str, route: &str) -> String {
    format!(
        "US.HWYBUNDLE.{:016X}",
        stable_segment_hash(&format!(
            "{}|{}",
            zone_id.trim(),
            normalise_designation(route)
        ))
    )
}

