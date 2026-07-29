//! Extracted helper `tier_pavement_debt_budget_gate_failures` from main.
use super::*;

pub(crate) fn tier_pavement_debt_budget_gate_failures(
    rows: &[TierPavementDebtBudgetRow],
    gap_rows: &[TierPavementSourceGapRow],
    exclusion_rows: &[TierPavementRouteStateExclusionRow],
    funding_rows: &[TierPavementRepairFundingAcceptanceRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_gap_rows = gap_rows
        .iter()
        .filter(|row| !pavement_gap_has_accepted_route_state_exclusion(row, exclusion_rows))
        .filter(|row| !pavement_gap_has_accepted_repair_funding(row, funding_rows))
        .collect::<Vec<_>>();
    if rows.len() != expected_gap_rows.len() {
        failures.push(format!(
            "pavement debt budget rows {} do not match non-excluded source-gap rows {}",
            rows.len(),
            expected_gap_rows.len()
        ));
    }
    let source_gap_bundles = expected_gap_rows
        .iter()
        .map(|row| row.segment_bundle_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for exclusion in exclusion_rows
        .iter()
        .filter(|row| row.validation_status == "pass")
    {
        if exclusion.exclusion_id.trim().is_empty()
            || exclusion.state.trim().is_empty()
            || exclusion.tier.trim().is_empty()
            || exclusion.route.trim().is_empty()
            || exclusion.segment_bundle_id.trim().is_empty()
            || exclusion.source_title.trim().is_empty()
            || exclusion.source_url_or_cache_artifact.trim().is_empty()
            || exclusion.capture_date.trim().is_empty()
            || exclusion.exclusion_basis.trim().is_empty()
            || exclusion.exclusion_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete pavement route-state exclusion",
                exclusion.exclusion_id
            ));
        }
        if exclusion.exclusion_status != "route-state-not-supported" {
            failures.push(format!(
                "{} has unsupported exclusion status {}",
                exclusion.exclusion_id, exclusion.exclusion_status
            ));
        }
        if exclusion.excluded_member_count == 0 {
            failures.push(format!(
                "{} has zero excluded pavement members",
                exclusion.exclusion_id
            ));
        }
        if !gap_rows.iter().any(|gap_row| {
            pavement_gap_has_accepted_route_state_exclusion(
                gap_row,
                std::slice::from_ref(exclusion),
            )
        }) {
            failures.push(format!(
                "{} does not match a pavement source-gap row",
                exclusion.exclusion_id
            ));
        }
    }
    for funding in funding_rows
        .iter()
        .filter(|row| row.validation_status == "pass")
    {
        if funding.acceptance_id.trim().is_empty()
            || funding.state.trim().is_empty()
            || funding.tier.trim().is_empty()
            || funding.route.trim().is_empty()
            || funding.segment_bundle_id.trim().is_empty()
            || funding.source_title.trim().is_empty()
            || funding.source_url_or_cache_artifact.trim().is_empty()
            || funding.capture_date.trim().is_empty()
            || funding.funding_basis.trim().is_empty()
            || funding.acceptance_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete pavement repair funding acceptance",
                funding.acceptance_id
            ));
        }
        if funding.acceptance_status != "accepted-full-cost-repair-funding" {
            failures.push(format!(
                "{} has unsupported acceptance status {}",
                funding.acceptance_id, funding.acceptance_status
            ));
        }
        if funding.committed_amount_m <= 0.0 || funding.covered_repair_cost_m <= 0.0 {
            failures.push(format!(
                "{} has non-positive accepted repair funding",
                funding.acceptance_id
            ));
        }
        if !gap_rows.iter().any(|gap_row| {
            pavement_gap_has_accepted_repair_funding(gap_row, std::slice::from_ref(funding))
        }) {
            failures.push(format!(
                "{} does not match a fully funded pavement source-gap row",
                funding.acceptance_id
            ));
        }
    }
    for row in rows {
        if row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.stitch_group_id.trim().is_empty()
            || row.debt_class.trim().is_empty()
            || row.budget_basis.trim().is_empty()
            || row.optimizer_penalty.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete pavement debt budget row",
                row.segment_bundle_id
            ));
        }
        if !source_gap_bundles.contains(row.segment_bundle_id.as_str()) {
            failures.push(format!(
                "{} has no matching pavement source-gap row",
                row.segment_bundle_id
            ));
        }
        if row.blocked_member_count == 0 {
            failures.push(format!(
                "{} has zero blocked pavement debt members",
                row.segment_bundle_id
            ));
        }
        if row.evidence_debt_units + row.repair_debt_units == 0 {
            failures.push(format!(
                "{} lacks evidence or repair debt units",
                row.segment_bundle_id
            ));
        }
        if row.total_debt_cost_m <= 0.0 {
            failures.push(format!(
                "{} has non-positive pavement debt cost",
                row.segment_bundle_id
            ));
        }
        if !row.segment_bundle_id.starts_with("US.HWYBUNDLE.") {
            failures.push(format!("{} is not a bundle id", row.segment_bundle_id));
        }
        if !row.stitch_group_id.starts_with("US.HWYSTITCH.") {
            failures.push(format!("{} is not a stitch id", row.stitch_group_id));
        }
        if row.validation_status != "review" {
            failures.push(format!(
                "{} must remain review until pavement debt closes",
                row.segment_bundle_id
            ));
        }
    }
    failures
}

