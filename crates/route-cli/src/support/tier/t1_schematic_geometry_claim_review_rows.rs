//! Helper `t1_schematic_geometry_claim_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_schematic_geometry_claim_review_rows(
    claim_rows: &[OptimizerClaimReviewRow],
    design_rows: &[T1DesignReviewCsvRow],
    policy_rows: &[T1DesignPolicyActionRow],
) -> Vec<T1SchematicGeometryClaimReviewRow> {
    let Some(claim_row) = claim_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T1"
            && row.blocker_family.contains("schematic_geometry")
            && row.total_claim_blockers > 0
    }) else {
        return Vec::new();
    };
    let Some(policy_row) = policy_rows
        .iter()
        .find(|row| row.action == "resolve-shared-segment-map-policy")
    else {
        return Vec::new();
    };
    let expected_routes = claim_row
        .representative_routes
        .split(';')
        .filter(|route| !route.trim().is_empty())
        .map(|route| route.trim().to_string())
        .collect::<std::collections::BTreeSet<_>>();

    let mut rows = design_rows
        .iter()
        .filter(|row| expected_routes.contains(&row.route))
        .filter(|row| {
            row.selected
                && row.design_status == "policy-review"
                && row.next_design_action == "resolve-shared-segment-map-policy"
                && row.beck_review_flag == "overlap-review"
        })
        .map(|row| T1SchematicGeometryClaimReviewRow {
            schematic_review_id: format!("T1SCHEMATIC-{}", stable_id_fragment(&row.route)),
            claim_review_id: claim_row.claim_review_id.clone(),
            route: row.route.clone(),
            design_role: row.design_role.clone(),
            design_status: row.design_status.clone(),
            beck_review_flag: row.beck_review_flag.clone(),
            overlap_corridors: row.overlap_corridors.clone(),
            policy_action: policy_row.action.clone(),
            required_policy: policy_row.required_policy.clone(),
            design_treatment: policy_row.design_treatment.clone(),
            gate_policy: policy_row.gate_policy.clone(),
            blocker_claims_before: claim_row.blocked_claims.clone(),
            blocker_claims_after: claim_row.blocked_claims.clone(),
            blocker_count_before: 2,
            blocker_count_after: 2,
            claim_blocker_delta: 0,
            review_decision: "shared-segment-map-policy-required".to_string(),
            next_artifact: "data/t1-shared-segment-map-policy.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
