//! Helper `merge_segment_identity`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn merge_segment_identity(
    builder: &mut NationalSegmentRegistryBuilder,
    segment_bundle_id: &str,
    stitch_group_id: &str,
    zone_id: &str,
    current_tier: &str,
    route: &str,
) {
    if builder.segment_bundle_id.is_empty() {
        builder.segment_bundle_id = segment_bundle_id.to_string();
    }
    if builder.stitch_group_id.is_empty() {
        builder.stitch_group_id = stitch_group_id.to_string();
    }
    if builder.zone_id.is_empty() {
        builder.zone_id = zone_id.to_string();
    }
    if builder.current_tier.is_empty() {
        builder.current_tier = current_tier.to_string();
    }
    if builder.route.is_empty() {
        builder.route = route.to_string();
    }
}

