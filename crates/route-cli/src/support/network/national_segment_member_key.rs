//! Helper `national_segment_member_key`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn national_segment_member_key(segment_bundle_id: &str, national_segment_id: &str) -> String {
    format!(
        "{}|{}",
        segment_bundle_id.trim(),
        national_segment_id.trim()
    )
}

