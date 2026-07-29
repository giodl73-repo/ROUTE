//! Helper `t2_national_bundle_readiness_audit_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_national_bundle_readiness_audit_gate_failures(
    rows: &[T2NationalBundleReadinessAuditRow],
    replay_rows: &[T2BundleReadinessReplayDecisionRow],
) -> Vec<String> {
    let expected = replay_rows
        .iter()
        .filter(|row| row.next_artifact == "data/national-segment-bundles.csv")
        .map(|row| row.replay_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "national bundle readiness audit has {} rows but expected {} replay rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.audit_id.trim().is_empty()
            || row.replay_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.readiness_class.trim().is_empty()
            || row.replay_decision.trim().is_empty()
            || row.bundle_status.trim().is_empty()
            || row.bundle_validation_status.trim().is_empty()
            || row.audit_decision.trim().is_empty()
            || row.audit_action.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete audit fields", row.route));
        }
        if !seen.insert(row.replay_id.clone()) {
            failures.push(format!("{} appears more than once", row.replay_id));
        }
        if !expected.contains(row.replay_id.as_str()) {
            failures.push(format!(
                "{} is not a national-bundle replay row",
                row.replay_id
            ));
        }
        if row.audit_decision == "pass"
            || row.audit_decision == "bound"
            || row.validation_status != "review"
        {
            failures.push(format!("{} audit promoted readiness", row.route));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.audit_decision != "held-for-structural-bundle-repair"
        {
            failures.push(format!(
                "{} audit carries qualification effects without held structural repair",
                row.route
            ));
        }
        if row.blocked_claims_before != "game;incident;publication;upgrade"
            || row.blocked_claims_after != "game;incident;publication;upgrade"
            || row.blocker_delta != 0
        {
            failures.push(format!("{} did not preserve claim blockers", row.route));
        }
        if !matches!(
            row.bundle_status.as_str(),
            "needs-stop-chain"
                | "needs-stitched-members"
                | "needs-terminal-stop"
                | "bundle-ready"
                | "missing-bundle-row"
                | "bundle-review"
        ) {
            failures.push(format!(
                "{} has unsupported bundle status {}",
                row.route, row.bundle_status
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from national bundle audit"));
        }
    }
    failures
}

