//! Helper `t1_evidence_window_can_promote`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_evidence_window_can_promote(row: &T1EvidenceWindowRow) -> bool {
    matches!(
        row.evidence_mode.trim(),
        "repeated_window" | "historical_archive"
    ) && !row.observation_start.trim().is_empty()
        && !row.observation_end.trim().is_empty()
        && row.event_count > 0
}

