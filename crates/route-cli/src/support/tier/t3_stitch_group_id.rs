//! Helper `t3_stitch_group_id`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_stitch_group_id(zone_id: &str, route: &str) -> String {
    format!(
        "US.HWYSTITCH.{:016X}",
        stable_segment_hash(&format!(
            "{}|{}",
            zone_id.trim(),
            normalise_designation(route)
        ))
    )
}
