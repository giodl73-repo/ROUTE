//! Helper `t1_evidence_window_has_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_evidence_window_has_contract(row: &T1EvidenceWindowRow) -> bool {
    !row.window_id.trim().is_empty()
        && !row.site_id.trim().is_empty()
        && !row.source_name.trim().is_empty()
        && matches!(
            row.evidence_mode.trim(),
            "snapshot_only" | "repeated_window" | "historical_archive" | "enrichment_blocker"
        )
        && !row.capture_started_at.trim().is_empty()
        && !row.capture_ended_at.trim().is_empty()
        && !row.raw_artifact.trim().is_empty()
        && !row.normalized_artifact.trim().is_empty()
        && row.freight_relevant_count <= row.event_count
        && !row.blocking_gap.trim().is_empty()
        && !row.next_step.trim().is_empty()
        && !row.review_artifact.trim().is_empty()
}
