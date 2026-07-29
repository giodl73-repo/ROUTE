//! Helper `tier_pavement_source_access_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_source_access_rows(
    docket_rows: &[TierPavementAcquisitionDocketRow],
    priority: &str,
) -> Vec<TierPavementSourceAccessRow> {
    docket_rows
        .iter()
        .filter(|row| row.source_priority.eq_ignore_ascii_case(priority))
        .map(|row| TierPavementSourceAccessRow {
            access_policy_id: format!("PAVEMENTACCESS-{}", stable_id_fragment(&row.task_id)),
            task_id: row.task_id.clone(),
            state: row.state.clone(),
            source_priority: row.source_priority.clone(),
            source_access_mode: "hpms-scoped-fetch".to_string(),
            mutation_mode: "scoped-cache-merge".to_string(),
            cache_targets: format!(
                "data/cache/hpms_2018.csv;data/cache/hpms_{}.csv",
                row.state.to_ascii_lowercase()
            ),
            fetch_command: row.fetch_command.clone(),
            preflight_gate: "route source-fetch-policy --gate".to_string(),
            postfetch_gate: row.verify_command.clone(),
            blocker_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect()
}

