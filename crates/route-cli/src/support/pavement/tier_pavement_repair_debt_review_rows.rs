//! Helper `tier_pavement_repair_debt_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_repair_debt_review_rows(
    unmatched_join_rows: &[TierPavementUnmatchedJoinReviewRow],
    debt_rows: &[TierPavementDebtBudgetRow],
    exclusion_rows: &[TierPavementRouteStateExclusionRow],
    funding_rows: &[TierPavementRepairFundingAcceptanceRow],
) -> Vec<TierPavementRepairDebtReviewRow> {
    let mut review_scope =
        std::collections::BTreeMap::<String, (&TierPavementUnmatchedJoinReviewRow, usize)>::new();
    for row in unmatched_join_rows {
        if row.source_priority == "A"
            && row.join_review_status == "repair-debt-not-source-join"
            && row.repair_required_member_count > 0
        {
            let excluded_count = pavement_excluded_repair_member_count(row, exclusion_rows);
            let funded_count = pavement_funded_repair_member_count(row, funding_rows);
            let expected_count = row
                .repair_required_member_count
                .saturating_sub(excluded_count)
                .saturating_sub(funded_count);
            if expected_count > 0 {
                review_scope.insert(row.state.clone(), (row, expected_count));
            }
        }
    }

    let mut consumed_by_state = std::collections::BTreeMap::<String, usize>::new();
    let mut rows = Vec::new();
    for debt in debt_rows
        .iter()
        .filter(|row| row.debt_class == "repair-debt" && row.validation_status == "review")
    {
        let matching_states = debt
            .affected_states
            .split(';')
            .map(str::trim)
            .filter_map(|state| {
                review_scope
                    .get(state)
                    .map(|scope| (state.to_string(), *scope))
            })
            .collect::<Vec<_>>();
        if matching_states.is_empty() {
            continue;
        }
        for (state, (join_row, _)) in matching_states {
            *consumed_by_state.entry(state.clone()).or_default() += debt.blocked_member_count;
            rows.push(TierPavementRepairDebtReviewRow {
                repair_review_id: format!(
                    "PAVEMENTREPAIRREVIEW-{}-{}",
                    stable_id_fragment(&state),
                    stable_id_fragment(&debt.segment_bundle_id)
                ),
                state,
                source_priority: join_row.source_priority.clone(),
                tier: debt.tier.clone(),
                route: debt.route.clone(),
                segment_bundle_id: debt.segment_bundle_id.clone(),
                stitch_group_id: debt.stitch_group_id.clone(),
                blocked_member_count: debt.blocked_member_count,
                repair_debt_units: debt.repair_debt_units,
                estimated_repair_cost_m: debt.estimated_repair_cost_m,
                repair_debt_status: "confirmed-repair-debt".to_string(),
                repair_decision: "hold-claims-until-repair-funded-or-design-downgraded".to_string(),
                evidence_acceptance_status: "not-accepted".to_string(),
                blocker_claims_before: join_row.blocker_claims_before.clone(),
                blocker_claims_after: join_row.blocker_claims_after.clone(),
                claim_blocker_delta: 0,
                next_action:
                    "prepare repair funding, downgrade, or exclusion decision before relief replay"
                        .to_string(),
                next_artifact: "data/tier-pavement-repair-debt-review.csv".to_string(),
                validation_status: "review".to_string(),
            });
        }
    }

    for (state, (join_row, expected_count)) in review_scope {
        let consumed = consumed_by_state.get(&state).copied().unwrap_or_default();
        if consumed < expected_count {
            rows.push(TierPavementRepairDebtReviewRow {
                repair_review_id: format!(
                    "PAVEMENTREPAIRREVIEW-{}-MISSING",
                    stable_id_fragment(&state)
                ),
                state,
                source_priority: join_row.source_priority.clone(),
                tier: "T2".to_string(),
                route: join_row.repair_required_routes.clone(),
                segment_bundle_id: "missing-debt-budget-row".to_string(),
                stitch_group_id: "missing-debt-budget-row".to_string(),
                blocked_member_count: expected_count - consumed,
                repair_debt_units: expected_count - consumed,
                estimated_repair_cost_m: 0.0,
                repair_debt_status: "missing-debt-budget-row".to_string(),
                repair_decision: "block-relief-until-debt-budget-row-exists".to_string(),
                evidence_acceptance_status: "not-accepted".to_string(),
                blocker_claims_before: join_row.blocker_claims_before.clone(),
                blocker_claims_after: join_row.blocker_claims_after.clone(),
                claim_blocker_delta: 0,
                next_action: "regenerate tier-pavement-debt-budget before review".to_string(),
                next_artifact: "data/tier-pavement-debt-budget.csv".to_string(),
                validation_status: "blocked".to_string(),
            });
        }
    }

    rows
}
