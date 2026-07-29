//! Helper `tier_pavement_funding_commitment_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_commitment_review_gate_failures(
    rows: &[TierPavementFundingCommitmentReviewRow],
    package_rows: &[TierPavementRepairFundingPackageRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = package_rows
        .iter()
        .filter(|row| {
            row.funding_package_status == "package-required"
                && row.funding_commitment_status == "unfunded"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .count();
    if expected > 0 && rows.len() != expected {
        failures.push(format!(
            "funding commitment review rows {} do not match funding package rows {}",
            rows.len(),
            expected
        ));
    }
    for row in rows {
        if row.commitment_review_id.trim().is_empty()
            || row.funding_package_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.funding_commitment_status.trim().is_empty()
            || row.accepted_commitment_artifact.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete commitment review row",
                row.state, row.route
            ));
        }
        if row.funding_commitment_status != "no-accepted-commitment-attached" {
            failures.push(format!(
                "{} {} has accepted or invalid funding commitment status {}",
                row.state, row.route, row.funding_commitment_status
            ));
        }
        if row.accepted_commitment_artifact != "none" {
            failures.push(format!(
                "{} {} attaches an unreviewed commitment artifact",
                row.state, row.route
            ));
        }
        if row.relief_eligibility != "not-eligible-for-relief" {
            failures.push(format!(
                "{} {} is relief eligible without accepted funding",
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

