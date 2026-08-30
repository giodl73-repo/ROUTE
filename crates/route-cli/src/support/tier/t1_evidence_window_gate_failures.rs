//! Helper `t1_evidence_window_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_evidence_window_gate_failures(rows: &[T1EvidenceWindowRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["no evidence-window rows found".to_string()];
    }

    let mut failures = Vec::new();
    for row in rows {
        if !t1_evidence_window_has_contract(row) {
            failures.push(format!(
                "{} lacks required source-window metadata",
                row.window_id
            ));
        }
        if row.promotion_eligible && !t1_evidence_window_can_promote(row) {
            failures.push(format!(
                "{} is promotion eligible without repeated-window or archive evidence",
                row.window_id
            ));
        }
        if row.evidence_mode.trim() == "snapshot_only" && row.promotion_eligible {
            failures.push(format!(
                "{} marks snapshot-only evidence as promotion eligible",
                row.window_id
            ));
        }
    }
    failures
}
