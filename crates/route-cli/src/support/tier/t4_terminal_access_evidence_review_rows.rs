//! Helper `t4_terminal_access_evidence_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_evidence_review_rows(
    contact_rows: &[T4TerminalContactEvidenceRow],
) -> Vec<T4TerminalAccessEvidenceReviewRow> {
    let mut rows = contact_rows
        .iter()
        .map(|row| T4TerminalAccessEvidenceReviewRow {
            review_id: format!("T4ACCESSREVIEW-{}", stable_id_fragment(&row.queue_id)),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district_seed: row.terminal_district_seed.clone(),
            terminal_district_seed_source: row.terminal_district_seed_source.clone(),
            evidence_status_before: row.evidence_status.clone(),
            review_decision: "held-source-needed".to_string(),
            review_reason:
                "terminal district seed assignment is not contact proof; non-seed source artifact still required"
                    .to_string(),
            source_action: "route-to-terminal-access-proof-acquisition".to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: row.next_artifact.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
