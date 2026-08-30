//! Helper `tier_pavement_hpms_scope_broadening_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_hpms_scope_broadening_rows(
    unmatched_join_rows: &[TierPavementUnmatchedJoinReviewRow],
    functional_systems: &[u8],
) -> Vec<TierPavementHpmsScopeBroadeningRow> {
    let systems = functional_systems
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",");
    unmatched_join_rows
        .iter()
        .map(|row| TierPavementHpmsScopeBroadeningRow {
            broadening_id: format!("PAVEMENTHPMSBROADEN-{}", stable_id_fragment(&row.state)),
            state: row.state.clone(),
            source_priority: row.source_priority.clone(),
            source_needed_routes: if row.source_needed_routes.trim().is_empty() {
                "none".to_string()
            } else {
                row.source_needed_routes.clone()
            },
            source_needed_member_count: row.source_needed_member_count,
            current_hpms_records_for_source_needed_routes: row.hpms_records_for_source_needed_routes,
            current_coverage_status: row.hpms_source_route_coverage.clone(),
            broadened_functional_systems: systems.clone(),
            broadened_fetch_command: if row.source_needed_member_count == 0 {
                "not-required-after-broadened-fetch".to_string()
            } else {
                format!(
                    "route fetch-hpms --states {} --functional-systems {}",
                    row.state, systems
                )
            },
            preflight_gate: "route source-fetch-policy --gate".to_string(),
            postfetch_gate:
                "route build --all-roads && route tier-pavement-docket --gate && route tier-pavement-source-gaps --gate"
                    .to_string(),
            evidence_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_before.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/tier-pavement-source-fetch-attempt.csv".to_string(),
            validation_status: if row.source_needed_member_count == 0 {
                "pass".to_string()
            } else {
                "review".to_string()
            },
        })
        .collect()
}
