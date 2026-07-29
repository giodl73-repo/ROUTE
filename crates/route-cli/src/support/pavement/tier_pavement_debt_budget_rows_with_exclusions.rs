//! Helper `tier_pavement_debt_budget_rows_with_exclusions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_debt_budget_rows_with_exclusions(
    gap_rows: &[TierPavementSourceGapRow],
    exclusion_rows: &[TierPavementRouteStateExclusionRow],
    funding_rows: &[TierPavementRepairFundingAcceptanceRow],
) -> Vec<TierPavementDebtBudgetRow> {
    gap_rows
        .iter()
        .filter(|row| !pavement_gap_has_accepted_route_state_exclusion(row, exclusion_rows))
        .filter(|row| !pavement_gap_has_accepted_repair_funding(row, funding_rows))
        .map(|row| {
            let needs_repair = row.blocker_statuses.contains("pavement-repair-required");
            let needs_evidence = row.blocker_statuses.contains("pavement-source-needed");
            let repair_debt_units = if needs_repair { row.blocker_count } else { 0 };
            let evidence_debt_units = if needs_evidence { row.blocker_count } else { 0 };
            let estimated_evidence_cost_m =
                round_cost_m(evidence_debt_units as f64 * PAVEMENT_EVIDENCE_COST_PER_MEMBER_M);
            let estimated_repair_cost_m =
                round_cost_m(repair_debt_units as f64 * PAVEMENT_REPAIR_COST_PER_MEMBER_M);
            let total_debt_cost_m =
                round_cost_m(estimated_evidence_cost_m + estimated_repair_cost_m);
            let debt_class = if needs_repair {
                "repair-debt"
            } else if needs_evidence {
                "evidence-debt"
            } else {
                "classification-debt"
            };
            let next_artifact = if needs_repair {
                "data/tier-pavement-docket.csv"
            } else {
                "data/tier-pavement-acquisition-plan.csv"
            };
            TierPavementDebtBudgetRow {
                tier: row.tier.clone(),
                route: row.route.clone(),
                region_id: row.region_id.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                stitch_group_id: row.stitch_group_id.clone(),
                debt_class: debt_class.to_string(),
                blocked_member_count: row.blocker_count,
                affected_states: row.affected_states.clone(),
                evidence_debt_units,
                repair_debt_units,
                estimated_evidence_cost_m,
                estimated_repair_cost_m,
                total_debt_cost_m,
                budget_basis: format!(
                    "planning proxy: evidence ${:.2}M/member; repair ${:.2}M/member until HPMS/state DOT unit costs replace defaults",
                    PAVEMENT_EVIDENCE_COST_PER_MEMBER_M, PAVEMENT_REPAIR_COST_PER_MEMBER_M
                ),
                optimizer_penalty: format!(
                    "subtract {:.2} budget-cost units from {} service claim until pavement debt closes",
                    total_debt_cost_m, row.segment_bundle_id
                ),
                next_artifact: next_artifact.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect()
}

