//! Helper `tier_pavement_unmatched_join_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_unmatched_join_review_gate_failures(
    rows: &[TierPavementUnmatchedJoinReviewRow],
    fetch_review_rows: &[TierPavementSourceFetchReviewRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if !fetch_review_rows.is_empty() && rows.len() != fetch_review_rows.len() {
        failures.push(format!(
            "unmatched join review rows {} do not match fetch review rows {}",
            rows.len(),
            fetch_review_rows.len()
        ));
    }
    for row in rows {
        if row.join_review_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.source_priority.trim().is_empty()
            || row.hpms_source_route_coverage.trim().is_empty()
            || row.join_review_status.trim().is_empty()
            || row.evidence_acceptance_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete unmatched-join review row",
                row.state
            ));
        }
        if row.evidence_acceptance_status != "not-accepted" {
            failures.push(format!("{} accepts evidence before review", row.state));
        }
        if row.claim_blocker_delta != 0 || row.blocker_claims_after != row.blocker_claims_before {
            failures.push(format!("{} reduces blockers before relief", row.state));
        }
        if row.source_gap_member_count
            != row.source_needed_member_count + row.repair_required_member_count
        {
            failures.push(format!(
                "{} source gap member count does not equal source-needed plus repair-required members",
                row.state
            ));
        }
        if row.source_needed_member_count > 0
            && row.hpms_records_for_source_needed_routes == 0
            && row.join_review_status != "hpms-scope-misses-source-needed-routes"
        {
            failures.push(format!(
                "{} has source-needed members without HPMS route records but wrong status",
                row.state
            ));
        }
        if !matches!(
            row.join_review_status.as_str(),
            "hpms-scope-misses-source-needed-routes"
                | "hpms-route-records-present-join-still-open"
                | "repair-debt-not-source-join"
                | "no-open-priority-a-pavement-gap"
        ) {
            failures.push(format!(
                "{} has invalid unmatched join status {}",
                row.state, row.join_review_status
            ));
        }
    }
    failures
}

