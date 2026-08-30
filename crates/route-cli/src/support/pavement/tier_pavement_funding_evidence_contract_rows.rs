//! Helper `tier_pavement_funding_evidence_contract_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_contract_rows(
    decision_rows: &[TierPavementDowngradeExclusionDecisionRow],
) -> Vec<TierPavementFundingEvidenceContractRow> {
    decision_rows
        .iter()
        .filter(|row| {
            row.downgrade_decision == "no-downgrade-selected"
                && row.exclusion_decision == "no-exclusion-selected"
                && row.service_status == "held-at-current-tier"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceContractRow {
            evidence_contract_id: format!(
                "PAVEMENTFUNDINGEVIDENCE-{}",
                stable_id_fragment(&row.downgrade_exclusion_decision_id)
            ),
            downgrade_exclusion_decision_id: row.downgrade_exclusion_decision_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            required_evidence:
                "accepted-programming-document-or-state-dot-commitment-covering-full-repair-cost"
                    .to_string(),
            minimum_commitment_amount_m: row.estimated_repair_cost_m,
            accepted_evidence_status: "source-needed".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding evidence artifact before relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-contract.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}
