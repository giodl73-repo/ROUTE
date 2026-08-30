//! Helper `t1_line_selector_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_line_selector_gate_failures(
    rows: &[T1LineSelectorRow],
    route_budget: usize,
    stop_budget: usize,
) -> Vec<String> {
    let selected = rows.iter().filter(|row| row.selected).collect::<Vec<_>>();
    let mut failures = Vec::new();
    if selected.is_empty() {
        failures.push("no T1 routes selected".to_string());
    }
    if selected.len() > route_budget {
        failures.push(format!(
            "selected {} routes over budget {route_budget}",
            selected.len()
        ));
    }
    let stop_refs = selected
        .iter()
        .map(|row| row.selected_stop_count)
        .sum::<usize>();
    if stop_refs > stop_budget {
        failures.push(format!(
            "selected {stop_refs} stop refs over budget {stop_budget}"
        ));
    }
    for row in rows
        .iter()
        .filter(|row| row.sla_pair_count > 0 && !row.selected)
    {
        failures.push(format!(
            "{} is required by SLA pair(s) {} but was not selected",
            row.route, row.sla_pairs
        ));
    }
    for row in rows {
        if row.constraint_debt_cost_m < 0.0 {
            failures.push(format!("{} has negative constraint debt cost", row.route));
        }
        if row.lifecycle_debt_cost_m < 0.0 {
            failures.push(format!("{} has negative lifecycle debt cost", row.route));
        }
        if row.constraint_penalty_score < 0.0 {
            failures.push(format!("{} has negative constraint penalty", row.route));
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
                "{} has constraint pressure without class summary and ledger artifact",
                row.route
            ));
        }
        if row.selected
            && row.hard_blocker_count > 0
            && !matches!(
                row.reason,
                "sla-required-budget-fit"
                    | "score-ranked-budget-fit"
                    | "score-exception-keep"
                    | "score-exception-conditional-keep"
            )
        {
            failures.push(format!(
                "{} selected with hard blockers but without explicit selector reason",
                row.route
            ));
        }
    }
    failures
}
