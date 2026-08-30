//! Helper `t3_zone_route_column_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_route_column_gate_failures(
    rows: &[T3ZoneRouteColumnRow],
    obligations: &[T3ZoneAccessObligationRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T3 zone route columns emitted".to_string());
        return failures;
    }

    let mut seen = std::collections::BTreeSet::<(String, String, String)>::new();
    for row in rows {
        if row.zone_id.trim().is_empty()
            || row.zone_name.trim().is_empty()
            || row.obligation_class.trim().is_empty()
            || row.route.trim().is_empty()
            || row.column_decision.trim().is_empty()
            || row.zone_role.trim().is_empty()
            || row.contact_requirement.trim().is_empty()
            || row.map_treatment.trim().is_empty()
            || row.selection_basis.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!(
                "{} {} {} has incomplete route-column fields",
                row.zone_id, row.obligation_class, row.route
            ));
        }
        if !seen.insert((
            row.zone_id.clone(),
            row.obligation_class.clone(),
            canonical_route_key(&row.route),
        )) {
            failures.push(format!(
                "{} {} {} has duplicate route-column row",
                row.zone_id, row.obligation_class, row.route
            ));
        }
        if row.column_decision == "selected" && row.current_score < T3_THRESHOLD {
            failures.push(format!(
                "{} {} selected below T3 threshold",
                row.zone_id, row.route
            ));
        }
        if row.constraint_debt_cost_m < 0.0 {
            failures.push(format!(
                "{} {} has negative constraint debt cost",
                row.zone_id, row.route
            ));
        }
        if row.lifecycle_debt_cost_m < 0.0 {
            failures.push(format!(
                "{} {} has negative lifecycle debt cost",
                row.zone_id, row.route
            ));
        }
        if row.constraint_penalty_score < 0.0 {
            failures.push(format!(
                "{} {} has negative constraint penalty",
                row.zone_id, row.route
            ));
        }
        if (row.hard_blocker_count > 0
            || row.claim_blocker_count > 0
            || row.constraint_debt_cost_m > 0.0
            || row.lifecycle_debt_cost_m > 0.0
            || row.constraint_penalty_score > 0.0)
            && (row.top_constraint_classes.trim().is_empty()
                || row.constraint_ledger_artifact.trim().is_empty())
        {
            failures.push(format!(
                "{} {} has constraint pressure without class summary and ledger artifact",
                row.zone_id, row.route
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} {} has invalid validation status {}",
                row.zone_id, row.route, row.validation_status
            ));
        }
    }

    for obligation in obligations
        .iter()
        .filter(|row| row.obligation_class == "regional-feeder-access")
    {
        let selected_count = rows
            .iter()
            .filter(|row| {
                row.zone_id == obligation.zone_id
                    && row.obligation_class == obligation.obligation_class
                    && row.column_decision == "selected"
            })
            .count();
        if selected_count == 0 {
            failures.push(format!(
                "{} regional-feeder-access has no selected route column",
                obligation.zone_id
            ));
        }
    }

    failures
}
