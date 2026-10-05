//! Helper `t2_relief_evidence_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_relief_evidence_gate_failures(rows: &[T2ReliefEvidenceRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_relief_evidence__" {
        let row = &rows[0];
        if row.relief_action != "relief-evidence-clear" || row.validation_status != "pass" {
            failures.push("relief evidence clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.relief_action.trim().is_empty()
            || row.evidence_basis.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
            || !matches!(row.validation_status.as_str(), "pass" | "review")
        {
            failures.push(format!(
                "{} has incomplete relief evidence docket",
                row.route
            ));
        }
    }
    failures
}
