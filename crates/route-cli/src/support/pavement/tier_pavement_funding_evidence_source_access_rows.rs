//! Helper `tier_pavement_funding_evidence_source_access_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_funding_evidence_source_access_rows(
    acquisition_rows: &[TierPavementFundingEvidenceAcquisitionRow],
) -> Vec<TierPavementFundingEvidenceSourceAccessRow> {
    acquisition_rows
        .iter()
        .filter(|row| {
            row.acquisition_status == "source-needed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceSourceAccessRow {
            source_access_id: format!(
                "PAVEMENTFUNDINGACCESS-{}",
                stable_id_fragment(&row.funding_evidence_acquisition_id)
            ),
            funding_evidence_acquisition_id: row.funding_evidence_acquisition_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            source_owner: row.candidate_source_owner.clone(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            live_fetch_status: "unsupported-no-safe-funding-commitment-fetcher".to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; issuing agency; committed amount; covered route and state"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            source_access_blocker:
                "no safe live funding-commitment fetch command exists; use manual/cached accepted programming or DOT commitment artifact"
                    .to_string(),
            evidence_artifact: "source-needed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action:
                "collect manual or cached accepted funding artifact before attachment and review"
                    .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-source-access.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}
