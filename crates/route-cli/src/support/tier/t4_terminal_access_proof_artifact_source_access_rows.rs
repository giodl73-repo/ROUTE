//! Helper `t4_terminal_access_proof_artifact_source_access_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_artifact_source_access_rows(
    target_rows: &[T4TerminalAccessProofArtifactAcquisitionTargetRow],
) -> Vec<T4TerminalAccessProofArtifactSourceAccessRow> {
    let mut rows = target_rows
        .iter()
        .filter(|row| {
            row.acquisition_status == "source-needed"
                && row.cache_status == "not-cached"
                && row.source_artifact_reference == "source-needed"
                && row.proof_acceptance_status == "not-accepted"
                && row.validation_status == "review"
        })
        .map(|row| T4TerminalAccessProofArtifactSourceAccessRow {
            source_access_id: format!(
                "T4ACCESSARTIFACTSOURCE-{}",
                stable_id_fragment(&row.acquisition_target_id)
            ),
            acquisition_target_id: row.acquisition_target_id.clone(),
            attachment_review_id: row.attachment_review_id.clone(),
            artifact_attachment_id: row.artifact_attachment_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_owner: row.candidate_source_owner.clone(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            cache_status: "not-cached".to_string(),
            live_fetch_status: "unsupported-no-safe-terminal-access-fetcher".to_string(),
            required_source_metadata: row.required_artifact_fields.clone(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            source_access_blocker:
                "no safe live terminal-access proof fetch command exists; use manual/cached non-seed proof artifact or add policy-compliant fetcher"
                    .to_string(),
            evidence_artifact: "source-needed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-artifact-source-access.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
