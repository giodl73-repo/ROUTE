//! Helper `build_t1_line_selector_csv`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn build_t1_line_selector_csv(rows: &[T1LineSelectorRow]) -> String {
    let mut csv = String::from(
        "route,tier,score,constraint_adjusted_score,rank,selected,selected_stop_count,top_city_stop_count,sla_pair_count,budget_cost,hard_blocker_count,claim_blocker_count,constraint_debt_cost_m,lifecycle_debt_cost_m,constraint_penalty_score,top_constraint_classes,constraint_ledger_artifact,decision,reason,selected_stops,top_city_stops,sla_pairs\n",
    );
    for row in rows {
        push_csv_line(
            &mut csv,
            &[
                &row.route,
                &row.tier,
                &format!("{:.1}", row.score),
                &format!("{:.1}", row.constraint_adjusted_score),
                &row.rank.to_string(),
                if row.selected { "true" } else { "false" },
                &row.selected_stop_count.to_string(),
                &row.top_city_stop_count.to_string(),
                &row.sla_pair_count.to_string(),
                &row.budget_cost.to_string(),
                &row.hard_blocker_count.to_string(),
                &row.claim_blocker_count.to_string(),
                &format!("{:.2}", row.constraint_debt_cost_m),
                &format!("{:.2}", row.lifecycle_debt_cost_m),
                &format!("{:.2}", row.constraint_penalty_score),
                &row.top_constraint_classes,
                &row.constraint_ledger_artifact,
                row.decision,
                row.reason,
                &row.selected_stops,
                &row.top_city_stops,
                &row.sla_pairs,
            ],
        );
    }
    csv
}

