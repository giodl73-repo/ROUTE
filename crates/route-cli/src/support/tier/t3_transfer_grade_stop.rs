//! Helper `t3_transfer_grade_stop`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_transfer_grade_stop(stop: &StopCandidateRow) -> bool {
    matches!(
        stop.requested_class.trim().to_ascii_uppercase().as_str(),
        "S1" | "S2" | "S3"
    )
}
