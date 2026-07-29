//! Helper `t1_schematic_geometry_claim_review_gate_failures` (support::tier).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_schematic_geometry_claim_review_gate_failures(
    rows: &[T1SchematicGeometryClaimReviewRow],
    claim_rows: &[OptimizerClaimReviewRow],
    design_rows: &[T1DesignReviewCsvRow],
    policy_rows: &[T1DesignPolicyActionRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let Some(claim_row) = claim_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T1"
            && row.blocker_family.contains("schematic_geometry")
            && row.total_claim_blockers > 0
    }) else {
        failures.push("missing T1 schematic geometry optimizer claim-review row".to_string());
        return failures;
    };
    if !policy_rows
        .iter()
        .any(|row| row.action == "resolve-shared-segment-map-policy")
    {
        failures.push("missing resolve-shared-segment-map-policy action".to_string());
    }
    let expected_routes = claim_row
        .representative_routes
        .split(';')
        .filter(|route| !route.trim().is_empty())
        .map(|route| route.trim().to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let eligible_routes = design_rows
        .iter()
        .filter(|row| expected_routes.contains(&row.route))
        .filter(|row| {
            row.selected
                && row.design_status == "policy-review"
                && row.next_design_action == "resolve-shared-segment-map-policy"
                && row.beck_review_flag == "overlap-review"
        })
        .map(|row| row.route.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if eligible_routes.len() != expected_routes.len() {
        failures.push(format!(
            "eligible T1 schematic routes = {}, expected {}",
            eligible_routes.len(),
            expected_routes.len()
        ));
    }
    if rows.len() != expected_routes.len() {
        failures.push(format!(
            "T1 schematic review has {} rows but expected {}",
            rows.len(),
            expected_routes.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.schematic_review_id.trim().is_empty()
            || row.claim_review_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.design_role.trim().is_empty()
            || row.design_status.trim().is_empty()
            || row.beck_review_flag.trim().is_empty()
            || row.overlap_corridors.trim().is_empty()
            || row.policy_action.trim().is_empty()
            || row.required_policy.trim().is_empty()
            || row.design_treatment.trim().is_empty()
            || row.gate_policy.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete schematic review fields",
                row.route
            ));
        }
        if !seen.insert(row.route.clone()) {
            failures.push(format!("{} appears more than once", row.route));
        }
        if !expected_routes.contains(&row.route) {
            failures.push(format!(
                "{} is not in the T1 schematic claim row",
                row.route
            ));
        }
        if row.claim_review_id != claim_row.claim_review_id
            || row.policy_action != "resolve-shared-segment-map-policy"
            || row.review_decision != "shared-segment-map-policy-required"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid schematic review state", row.route));
        }
        if row.blocker_claims_before != claim_row.blocked_claims
            || row.blocker_claims_after != claim_row.blocked_claims
            || row.blocker_count_before != 2
            || row.blocker_count_after != 2
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} reduced schematic claim blockers", row.route));
        }
    }
    for expected_route in expected_routes {
        if !seen.contains(&expected_route) {
            failures.push(format!(
                "{expected_route} missing from T1 schematic claim review"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != claim_row.total_claim_blockers {
        failures.push(format!(
            "T1 schematic review preserves {total_after} blockers but claim row has {}",
            claim_row.total_claim_blockers
        ));
    }
    failures
}

