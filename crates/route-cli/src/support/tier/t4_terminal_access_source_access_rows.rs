//! Helper `t4_terminal_access_source_access_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_source_access_rows(
    review_rows: &[T4TerminalAccessProofReviewRow],
) -> Vec<T4TerminalAccessSourceAccessRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| row.review_decision == "held-no-source-artifact")
        .map(|row| T4TerminalAccessSourceAccessRow {
            source_access_id: format!(
                "T4ACCESSSOURCE-{}",
                stable_id_fragment(&row.proof_review_id)
            ),
            proof_review_id: row.proof_review_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_owner: "terminal operator, port authority, state DOT, or public terminal map".to_string(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            live_fetch_status: "unsupported-no-safe-terminal-access-fetcher".to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; route; terminal; connector; route-to-terminal contact statement"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            source_access_blocker:
                "no safe live terminal-access proof fetch command exists; use manual/cached non-seed proof artifact or add policy-compliant fetcher"
                    .to_string(),
            evidence_artifact: "source-needed".to_string(),
            proof_acceptance_status: row.proof_acceptance_status.clone(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
