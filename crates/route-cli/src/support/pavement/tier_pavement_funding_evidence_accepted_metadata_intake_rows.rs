//! Helper `tier_pavement_funding_evidence_accepted_metadata_intake_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_accepted_metadata_intake_rows(
    access_rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow],
) -> Vec<TierPavementFundingEvidenceAcceptedMetadataIntakeRow> {
    access_rows
        .iter()
        .filter(|row| {
            row.access_mode == "manual-or-cached-source-needed"
                && row.cache_status == "not-cached"
                && row.evidence_artifact == "source-needed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceAcceptedMetadataIntakeRow {
            accepted_metadata_intake_id: format!(
                "PAVEMENTFUNDINGACCEPTEDMETAINTAKE-{}",
                stable_id_fragment(&row.accepted_metadata_source_access_id)
            ),
            accepted_metadata_source_access_id: row.accepted_metadata_source_access_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            required_source_metadata: row.required_source_metadata.clone(),
            intake_status: "artifact-required".to_string(),
            cache_status: "not-cached".to_string(),
            evidence_artifact: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            intake_blocker: "accepted funding artifact metadata has not been captured".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact metadata".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-metadata-intake.csv"
                .to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

