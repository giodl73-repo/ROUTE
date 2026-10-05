//! Helper `t2_beck_label_density_review_gate_failures` (support::tier).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_label_density_review_gate_failures(
    rows: &[T2BeckLabelDensityReviewRow],
    claim_rows: &[OptimizerClaimReviewRow],
    diagnostics: &[route_map::BeckT2DiagnosticRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let Some(claim_row) = claim_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T2"
            && row.blocker_family == "beck_label_density"
            && row.total_claim_blockers > 0
    }) else {
        failures.push("missing T2 Beck label-density optimizer claim-review row".to_string());
        return failures;
    };
    let expected_routes = claim_row
        .representative_routes
        .split(';')
        .filter(|route| !route.trim().is_empty())
        .map(route_display_key)
        .collect::<std::collections::BTreeSet<_>>();
    let eligible_routes = diagnostics
        .iter()
        .filter(|row| {
            matches!(
                row.review_flag,
                "dense-label-review" | "dense-transfer-review"
            )
        })
        .map(|row| route_display_key(row.corridor))
        .filter(|route| expected_routes.contains(route))
        .collect::<std::collections::BTreeSet<_>>();
    if eligible_routes.len() != expected_routes.len() {
        failures.push(format!(
            "eligible T2 label-density routes = {}, expected {}",
            eligible_routes.len(),
            expected_routes.len()
        ));
    }
    if rows.len() != expected_routes.len() {
        failures.push(format!(
            "T2 label-density review has {} rows but expected {}",
            rows.len(),
            expected_routes.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.label_review_id.trim().is_empty()
            || row.claim_review_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.trunk.trim().is_empty()
            || row.start_trunk.trim().is_empty()
            || row.end_trunk.trim().is_empty()
            || row.service_class.trim().is_empty()
            || row.service_label.trim().is_empty()
            || row.review_flag.trim().is_empty()
            || row.density_basis.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete label-density review fields",
                row.route
            ));
        }
        if !seen.insert(row.route.clone()) {
            failures.push(format!("{} appears more than once", row.route));
        }
        if !expected_routes.contains(&row.route) {
            failures.push(format!(
                "{} is not in the T2 label-density claim row",
                row.route
            ));
        }
        if row.claim_review_id != claim_row.claim_review_id
            || !matches!(
                row.review_flag.as_str(),
                "dense-label-review" | "dense-transfer-review"
            )
            || row.review_decision != "label-density-policy-required"
            || row.validation_status != "review"
        {
            failures.push(format!("{} has invalid label-density state", row.route));
        }
        if row.blocker_claims_before != claim_row.blocked_claims
            || row.blocker_claims_after != claim_row.blocked_claims
            || row.blocker_count_before != 1
            || row.blocker_count_after != 1
            || row.claim_blocker_delta != 0
        {
            failures.push(format!(
                "{} reduced label-density claim blockers",
                row.route
            ));
        }
    }
    for expected_route in expected_routes {
        if !seen.contains(&expected_route) {
            failures.push(format!(
                "{expected_route} missing from T2 label-density review"
            ));
        }
    }
    let total_after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    if total_after != claim_row.total_claim_blockers {
        failures.push(format!(
            "T2 label-density review preserves {total_after} blockers but claim row has {}",
            claim_row.total_claim_blockers
        ));
    }
    failures
}
