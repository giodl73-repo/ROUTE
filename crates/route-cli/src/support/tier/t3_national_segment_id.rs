//! Helper `t3_national_segment_id`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_national_segment_id(zone_id: &str, route: &str) -> String {
    format!(
        "US.HWYSEG.{:016X}",
        stable_segment_hash(&format!(
            "{}|{}",
            zone_id.trim(),
            normalise_designation(route)
        ))
    )
}

