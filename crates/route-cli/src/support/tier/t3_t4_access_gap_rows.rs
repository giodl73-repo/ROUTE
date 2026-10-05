//! Helper `t3_t4_access_gap_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_t4_access_gap_rows(
    route_rows: &[T3ZoneRouteColumnRow],
    terminal_rows: &[T4TerminalAccessColumnRow],
) -> Vec<T3T4AccessGapRow> {
    let mut rows = Vec::new();

    for row in route_rows
        .iter()
        .filter(|row| row.next_artifact == "data/t3-t4-access-gaps.csv")
    {
        let (gap_class, repair_action) = match row.column_decision.as_str() {
            "review" => (
                "below-threshold-feeder",
                "prove-terminal-evidence-or-keep-t4",
            ),
            _ => ("route-column-review", "review-route-column-disposition"),
        };
        rows.push(T3T4AccessGapRow {
            gap_id: format!(
                "T3GAP-{}-{}",
                canonical_route_key(&row.zone_id),
                canonical_route_key(&row.route)
            ),
            source_surface: "t3-zone-route-columns".to_string(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            current_score: row.current_score,
            constraint_adjusted_score: row.constraint_adjusted_score,
            hard_blocker_count: row.hard_blocker_count,
            claim_blocker_count: row.claim_blocker_count,
            constraint_debt_cost_m: row.constraint_debt_cost_m,
            lifecycle_debt_cost_m: row.lifecycle_debt_cost_m,
            constraint_penalty_score: row.constraint_penalty_score,
            top_constraint_classes: row.top_constraint_classes.clone(),
            constraint_ledger_artifact: row.constraint_ledger_artifact.clone(),
            promise_horizon_hours: row.promise_horizon_hours,
            gap_class: gap_class.to_string(),
            gap_reason: row.selection_basis.clone(),
            required_evidence: row.contact_requirement.clone(),
            repair_action: repair_action.to_string(),
            next_artifact: "data/t3-zone-map-diagnostics.csv".to_string(),
            upward_pressure_allowed: false,
            validation_status: "review".to_string(),
        });
    }

    for row in terminal_rows
        .iter()
        .filter(|row| row.next_artifact == "data/t3-t4-access-gaps.csv")
    {
        let (gap_class, repair_action) = match row.column_decision.as_str() {
            "zone-assignment-needed" => {
                ("zone-assignment-needed", "assign-zone-or-terminal-district")
            }
            "terminal-review" => (
                "terminal-evidence-needed",
                "prove-terminal-access-or-keep-local",
            ),
            _ => (
                "terminal-column-review",
                "review-terminal-column-disposition",
            ),
        };
        rows.push(T3T4AccessGapRow {
            gap_id: format!(
                "T4GAP-{}-{}",
                canonical_route_key(&row.zone_id),
                canonical_route_key(&row.route)
            ),
            source_surface: "t4-terminal-access-columns".to_string(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            current_score: row.current_score,
            constraint_adjusted_score: row.constraint_adjusted_score,
            hard_blocker_count: row.hard_blocker_count,
            claim_blocker_count: row.claim_blocker_count,
            constraint_debt_cost_m: row.constraint_debt_cost_m,
            lifecycle_debt_cost_m: row.lifecycle_debt_cost_m,
            constraint_penalty_score: row.constraint_penalty_score,
            top_constraint_classes: row.top_constraint_classes.clone(),
            constraint_ledger_artifact: row.constraint_ledger_artifact.clone(),
            promise_horizon_hours: row.promise_horizon_hours,
            gap_class: gap_class.to_string(),
            gap_reason: row.selection_basis.clone(),
            required_evidence: row.evidence_required.clone(),
            repair_action: repair_action.to_string(),
            next_artifact: if row.column_decision == "terminal-review" {
                "data/t4-terminal-contact-evidence.csv".to_string()
            } else {
                "data/t3-zone-map-diagnostics.csv".to_string()
            },
            upward_pressure_allowed: false,
            validation_status: "review".to_string(),
        });
    }

    rows.sort_by(|a, b| {
        a.gap_class
            .cmp(&b.gap_class)
            .then_with(|| a.zone_id.cmp(&b.zone_id))
            .then_with(|| b.current_score.total_cmp(&a.current_score))
            .then_with(|| a.route.cmp(&b.route))
            .then_with(|| a.source_surface.cmp(&b.source_surface))
    });
    rows
}
