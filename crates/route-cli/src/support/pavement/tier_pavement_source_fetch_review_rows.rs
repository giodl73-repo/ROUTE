//! Helper `tier_pavement_source_fetch_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_source_fetch_review_rows(
    fetch_attempt_rows: &[TierPavementSourceFetchAttemptRow],
    docket_rows: &[TierPavementAcquisitionDocketRow],
    source_gap_rows: &[TierPavementSourceGapRow],
) -> Vec<TierPavementSourceFetchReviewRow> {
    let docket_by_task = docket_rows
        .iter()
        .map(|row| (row.task_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();

    fetch_attempt_rows
        .iter()
        .map(|row| {
            let docket_row = docket_by_task.get(row.task_id.as_str()).copied();
            let pre_review_blocked_member_count =
                docket_row.map(|row| row.blocked_member_count).unwrap_or(0);
            let postfetch_unresolved_member_count = docket_row
                .map(|docket_row| {
                    if pavement_source_gap_still_open_for_task(docket_row, source_gap_rows) {
                        docket_row.blocked_member_count
                    } else {
                        0
                    }
                })
                .unwrap_or(0);
            let (join_review_status, next_action, next_artifact) =
                if row.cache_record_count == 0
                    || row.fetch_result_status == "fetch-failed-or-empty-cache"
                {
                    (
                        "fetch-repair-needed",
                        "repair scoped HPMS fetch or attach state DOT pavement source before evidence review",
                        "data/tier-pavement-source-access.csv",
                    )
                } else if postfetch_unresolved_member_count > 0 {
                    (
                        "cache-populated-source-gap-still-open",
                        "review unmatched HPMS joins or attach state DOT pavement condition evidence before blocker relief",
                        "data/tier-pavement-docket.csv",
                    )
                } else {
                    (
                        "source-gap-closed-pending-evidence-review",
                        "review rebuilt pavement docket before blocker relief replay",
                        "data/tier-pavement-docket.csv",
                    )
                };
            TierPavementSourceFetchReviewRow {
                review_id: format!("PAVEMENTFETCHREVIEW-{}", stable_id_fragment(&row.task_id)),
                fetch_attempt_id: row.fetch_attempt_id.clone(),
                task_id: row.task_id.clone(),
                state: row.state.clone(),
                source_priority: row.source_priority.clone(),
                cache_record_count: row.cache_record_count,
                fetch_result_status: row.fetch_result_status.clone(),
                pre_review_blocked_member_count,
                postfetch_unresolved_member_count,
                join_review_status: join_review_status.to_string(),
                evidence_acceptance_status: "not-accepted".to_string(),
                blocker_claims_before: row.blocker_claims_before.clone(),
                blocker_claims_after: row.blocker_claims_after.clone(),
                claim_blocker_delta: 0,
                next_action: next_action.to_string(),
                next_artifact: next_artifact.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect()
}
