//! Helper `tier_pavement_source_fetch_attempt_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_source_fetch_attempt_rows(
    source_access_rows: &[TierPavementSourceAccessRow],
) -> Result<Vec<TierPavementSourceFetchAttemptRow>> {
    let mut rows = Vec::new();
    for row in source_access_rows {
        let cache_target = row
            .cache_targets
            .split(';')
            .map(str::trim)
            .find(|target| {
                target.ends_with(&format!("hpms_{}.csv", row.state.to_ascii_lowercase()))
            })
            .unwrap_or("data/cache/hpms_2018.csv");
        let cache_record_count = count_csv_records(Path::new(cache_target))?;
        let fetch_result_status = if cache_record_count == 0 {
            "fetch-failed-or-empty-cache"
        } else {
            "cache-populated-unreviewed"
        };
        rows.push(TierPavementSourceFetchAttemptRow {
            fetch_attempt_id: format!("PAVEMENTFETCH-{}", stable_id_fragment(&row.task_id)),
            access_policy_id: row.access_policy_id.clone(),
            task_id: row.task_id.clone(),
            state: row.state.clone(),
            source_priority: row.source_priority.clone(),
            fetch_command: row.fetch_command.clone(),
            cache_target: cache_target.to_string(),
            cache_record_count,
            fetch_result_status: fetch_result_status.to_string(),
            evidence_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_before.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        });
    }
    Ok(rows)
}
