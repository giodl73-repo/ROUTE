//! Helper `tier_segment_bundle_role`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_segment_bundle_role(row: &TierSegmentCandidateRow) -> &'static str {
    if row.member_role == "stitched-member" {
        "stitched-service"
    } else {
        "single-segment"
    }
}
