//! Helper `tier_pavement_repair_debt_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_repair_debt_review_gate_failures(
    rows: &[TierPavementRepairDebtReviewRow],
    unmatched_join_rows: &[TierPavementUnmatchedJoinReviewRow],
    exclusion_rows: &[TierPavementRouteStateExclusionRow],
    funding_rows: &[TierPavementRepairFundingAcceptanceRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_by_state = unmatched_join_rows
        .iter()
        .filter(|row| {
            row.source_priority == "A"
                && row.join_review_status == "repair-debt-not-source-join"
                && row.repair_required_member_count > 0
        })
        .filter_map(|row| {
            let excluded_count = pavement_excluded_repair_member_count(row, exclusion_rows);
            let funded_count = pavement_funded_repair_member_count(row, funding_rows);
            let expected_count = row
                .repair_required_member_count
                .saturating_sub(excluded_count)
                .saturating_sub(funded_count);
            (expected_count > 0).then(|| (row.state.clone(), expected_count))
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    if !expected_by_state.is_empty() && rows.is_empty() {
        failures.push("no priority-A pavement repair debt review rows emitted".to_string());
        return failures;
    }

    let mut reviewed_by_state = std::collections::BTreeMap::<String, usize>::new();
    for row in rows {
        if row.repair_review_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.source_priority.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.stitch_group_id.trim().is_empty()
            || row.repair_debt_status.trim().is_empty()
            || row.repair_decision.trim().is_empty()
            || row.evidence_acceptance_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete repair review row",
                row.state, row.route
            ));
        }
        if !expected_by_state.contains_key(&row.state) {
            failures.push(format!(
                "{} is not in priority-A repair review scope",
                row.state
            ));
        }
        if row.repair_debt_status != "confirmed-repair-debt" {
            failures.push(format!(
                "{} {} has invalid repair debt status {}",
                row.state, row.route, row.repair_debt_status
            ));
        }
        if row.evidence_acceptance_status != "not-accepted" {
            failures.push(format!(
                "{} {} accepts evidence before relief",
                row.state, row.route
            ));
        }
        if row.claim_blocker_delta != 0 || row.blocker_claims_after != row.blocker_claims_before {
            failures.push(format!(
                "{} {} reduces blockers before relief",
                row.state, row.route
            ));
        }
        if row.repair_debt_units != row.blocked_member_count {
            failures.push(format!(
                "{} {} repair units do not match blocked members",
                row.state, row.route
            ));
        }
        if row.estimated_repair_cost_m <= 0.0 {
            failures.push(format!(
                "{} {} lacks positive repair cost",
                row.state, row.route
            ));
        }
        *reviewed_by_state.entry(row.state.clone()).or_default() += row.blocked_member_count;
    }
    for (state, expected) in expected_by_state {
        let reviewed = reviewed_by_state.get(&state).copied().unwrap_or_default();
        if reviewed != expected {
            failures.push(format!(
                "{} reviewed repair member count {} does not match unmatched join review {}",
                state, reviewed, expected
            ));
        }
    }
    failures
}

