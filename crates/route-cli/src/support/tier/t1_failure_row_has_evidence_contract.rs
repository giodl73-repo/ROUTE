//! Helper `t1_failure_row_has_evidence_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_failure_row_has_evidence_contract(row: &T1FailureRow) -> bool {
    let status = row.source_status.trim().to_ascii_lowercase();
    let status_is_labeled = matches!(status.as_str(), "empirical" | "modeled" | "source_needed");
    let confidence = row.confidence.trim().to_ascii_lowercase();
    let confidence_is_labeled =
        matches!(confidence.as_str(), "high" | "medium" | "low" | "unknown");
    let source_needed_has_gap = status != "source_needed" || !row.blocking_gap.trim().is_empty();

    !row.site_id.trim().is_empty()
        && !row.intersection.trim().is_empty()
        && !row.failure_mode.trim().is_empty()
        && status_is_labeled
        && confidence_is_labeled
        && !row.current_artifact.trim().is_empty()
        && !row.next_evidence_step.trim().is_empty()
        && source_needed_has_gap
}

