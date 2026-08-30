//! Helper `tier_pavement_funding_evidence_contract_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_contract_gate_failures(
    rows: &[TierPavementFundingEvidenceContractRow],
    decision_rows: &[TierPavementDowngradeExclusionDecisionRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = decision_rows
        .iter()
        .filter(|row| {
            row.downgrade_decision == "no-downgrade-selected"
                && row.exclusion_decision == "no-exclusion-selected"
                && row.service_status == "held-at-current-tier"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .count();
    if expected > 0 && rows.len() != expected {
        failures.push(format!(
            "funding evidence contract rows {} do not match downgrade/exclusion rows {}",
            rows.len(),
            expected
        ));
    }
    for row in rows {
        if row.evidence_contract_id.trim().is_empty()
            || row.downgrade_exclusion_decision_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.accepted_evidence_status.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete funding evidence contract row",
                row.state, row.route
            ));
        }
        if row.accepted_evidence_status != "source-needed" {
            failures.push(format!(
                "{} {} accepts funding evidence prematurely",
                row.state, row.route
            ));
        }
        if row.relief_eligibility != "not-eligible-for-relief" {
            failures.push(format!(
                "{} {} is relief eligible before evidence",
                row.state, row.route
            ));
        }
        if row.claim_blocker_delta != 0 {
            failures.push(format!(
                "{} {} reduces blockers before relief",
                row.state, row.route
            ));
        }
        if row.minimum_commitment_amount_m < row.estimated_repair_cost_m
            || row.minimum_commitment_amount_m <= 0.0
        {
            failures.push(format!(
                "{} {} has insufficient minimum commitment",
                row.state, row.route
            ));
        }
        if row.validation_status != "held" {
            failures.push(format!("{} {} is not held", row.state, row.route));
        }
    }
    failures
}
