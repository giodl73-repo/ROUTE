//! Helper `t2_bundle_readiness_repair_evidence_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_readiness_repair_evidence_gate_failures(
    rows: &[T2BundleReadinessRepairEvidenceRow],
    repair_rows: &[T2BundleReadinessRepairDocketRow],
) -> Vec<String> {
    let expected = repair_rows
        .iter()
        .map(|row| row.repair_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "readiness repair evidence has {} rows but expected {} repair tasks",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.evidence_id.trim().is_empty()
            || row.repair_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.readiness_class.trim().is_empty()
            || row.evidence_artifact.trim().is_empty()
            || row.evidence_status.trim().is_empty()
            || row.evidence_summary.trim().is_empty()
            || row.evidence_decision.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.blocks_claims.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete repair evidence fields",
                row.route
            ));
        }
        if !seen.insert(row.repair_id.clone()) {
            failures.push(format!("{} appears more than once", row.repair_id));
        }
        if !expected.contains(row.repair_id.as_str()) {
            failures.push(format!("{} is not a readiness repair task", row.repair_id));
        }
        if row.evidence_decision != "held-for-readiness-replay" {
            failures.push(format!("{} evidence probe promoted readiness", row.route));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.evidence_decision != "held-for-readiness-replay"
        {
            failures.push(format!(
                "{} repair evidence carries qualification effects without held replay",
                row.route
            ));
        }
        if row.blocks_claims != "game;incident;publication;upgrade" {
            failures.push(format!("{} does not preserve claim blockers", row.route));
        }
        if row.validation_status != "review" {
            failures.push(format!("{} repair evidence must remain review", row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} missing from readiness repair evidence"
            ));
        }
    }
    failures
}

