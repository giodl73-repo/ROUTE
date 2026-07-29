//! Helper `t2_bundle_readiness_replay_decision_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_readiness_replay_decision_gate_failures(
    rows: &[T2BundleReadinessReplayDecisionRow],
    evidence_rows: &[T2BundleReadinessRepairEvidenceRow],
    delta_rows: &[T2BundleOverlayRepairDeltaRow],
) -> Vec<String> {
    let expected = evidence_rows
        .iter()
        .map(|row| row.evidence_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let delta_bundles = delta_rows
        .iter()
        .map(|row| row.segment_bundle_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "readiness replay decisions has {} rows but expected {} evidence rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for evidence in evidence_rows {
        if !delta_bundles.contains(evidence.segment_bundle_id.as_str()) {
            failures.push(format!(
                "{} evidence row has no repair delta row",
                evidence.route
            ));
        }
    }
    for row in rows {
        if row.replay_id.trim().is_empty()
            || row.evidence_id.trim().is_empty()
            || row.delta_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.readiness_class.trim().is_empty()
            || row.evidence_status.trim().is_empty()
            || row.delta_replay_decision.trim().is_empty()
            || row.replay_decision.trim().is_empty()
            || row.replay_action.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete replay decision fields",
                row.route
            ));
        }
        if !seen.insert(row.evidence_id.clone()) {
            failures.push(format!("{} appears more than once", row.evidence_id));
        }
        if !expected.contains(row.evidence_id.as_str()) {
            failures.push(format!(
                "{} is not a readiness evidence row",
                row.evidence_id
            ));
        }
        if row.replay_decision == "bound" || row.delta_replay_decision == "bound" {
            failures.push(format!("{} readiness replay promoted a claim", row.route));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.replay_decision != "held-for-bundle-replay"
        {
            failures.push(format!(
                "{} readiness replay carries qualification effects without held replay",
                row.route
            ));
        }
        if row.blocked_claims_before != "game;incident;publication;upgrade"
            || row.blocked_claims_after != "game;incident;publication;upgrade"
            || row.blocker_delta != 0
        {
            failures.push(format!("{} did not preserve claim blockers", row.route));
        }
        if row.validation_status != "review" {
            failures.push(format!("{} replay decision must remain review", row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} missing from readiness replay decisions"
            ));
        }
    }
    failures
}

