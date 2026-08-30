//! Helper `tier_pavement_downgrade_exclusion_decision_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_downgrade_exclusion_decision_gate_failures(
    rows: &[TierPavementDowngradeExclusionDecisionRow],
    commitment_rows: &[TierPavementFundingCommitmentReviewRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = commitment_rows
        .iter()
        .filter(|row| {
            row.funding_commitment_status == "no-accepted-commitment-attached"
                && row.accepted_commitment_artifact == "none"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .count();
    if expected > 0 && rows.len() != expected {
        failures.push(format!(
            "downgrade/exclusion decision rows {} do not match commitment review rows {}",
            rows.len(),
            expected
        ));
    }
    for row in rows {
        if row.downgrade_exclusion_decision_id.trim().is_empty()
            || row.commitment_review_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.downgrade_decision.trim().is_empty()
            || row.exclusion_decision.trim().is_empty()
            || row.service_status.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete downgrade/exclusion row",
                row.state, row.route
            ));
        }
        if row.downgrade_decision != "no-downgrade-selected" {
            failures.push(format!(
                "{} {} downgrades without authorization",
                row.state, row.route
            ));
        }
        if row.exclusion_decision != "no-exclusion-selected" {
            failures.push(format!(
                "{} {} excludes without authorization",
                row.state, row.route
            ));
        }
        if row.service_status != "held-at-current-tier" {
            failures.push(format!(
                "{} {} changes service status",
                row.state, row.route
            ));
        }
        if row.relief_eligibility != "not-eligible-for-relief" {
            failures.push(format!(
                "{} {} is relief eligible without funding or authorized downgrade/exclusion",
                row.state, row.route
            ));
        }
        if row.claim_blocker_delta != 0 {
            failures.push(format!(
                "{} {} reduces blockers before relief",
                row.state, row.route
            ));
        }
        if row.estimated_repair_cost_m <= 0.0 {
            failures.push(format!(
                "{} {} lacks repair cost magnitude",
                row.state, row.route
            ));
        }
        if row.validation_status != "held" {
            failures.push(format!("{} {} is not held", row.state, row.route));
        }
    }
    failures
}
