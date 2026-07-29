//! Helper `t2_stitched_member_registry_handoff_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_registry_handoff_gate_failures(
    rows: &[T2StitchedMemberRegistryHandoffRow],
    audit_rows: &[T2NationalBundleReadinessAuditRow],
) -> Vec<String> {
    let expected = audit_rows
        .iter()
        .filter(|row| {
            row.readiness_class == "stitched-member"
                && row.next_artifact == "data/national-segment-registry.csv"
        })
        .map(|row| row.audit_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "stitched member registry handoff has {} rows but expected {} audit rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.handoff_id.trim().is_empty()
            || row.audit_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.stitch_group_id.trim().is_empty()
            || row.handoff_decision.trim().is_empty()
            || row.handoff_action.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete stitched handoff fields",
                row.route
            ));
        }
        if !seen.insert(row.audit_id.clone()) {
            failures.push(format!("{} appears more than once", row.audit_id));
        }
        if !expected.contains(row.audit_id.as_str()) {
            failures.push(format!(
                "{} is not a stitched-member audit row",
                row.audit_id
            ));
        }
        if row.current_registry_member_count >= row.required_member_min
            || row.handoff_decision == "pass"
            || row.handoff_decision == "bound"
            || row.validation_status != "review"
        {
            failures.push(format!("{} handoff promoted stitched readiness", row.route));
        }
        if row.candidate_route_member_count == 0 {
            failures.push(format!(
                "{} has no route-level candidate evidence",
                row.route
            ));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.handoff_decision != "held-for-member-expansion"
        {
            failures.push(format!(
                "{} handoff carries qualification effects without member-expansion hold",
                row.route
            ));
        }
        if row.blocked_claims_before != "game;incident;publication;upgrade"
            || row.blocked_claims_after != "game;incident;publication;upgrade"
            || row.blocker_delta != 0
        {
            failures.push(format!("{} did not preserve claim blockers", row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} missing from stitched registry handoff"
            ));
        }
    }
    failures
}

