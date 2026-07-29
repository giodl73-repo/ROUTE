//! Helper `t1_sla_candidate_pair_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_sla_candidate_pair_gate_failures(
    rows: &[T1SlaCandidatePairRow],
    selected_rows: &[T1SlaPairRow],
    selected_budget: usize,
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T1 SLA candidate pairs emitted".to_string());
        return failures;
    }
    let selected_count = rows.iter().filter(|row| row.portfolio_selected).count();
    if selected_count != selected_budget {
        failures.push(format!(
            "selected portfolio has {selected_count} rows, expected {selected_budget}"
        ));
    }
    let row_pair_ids = rows
        .iter()
        .map(|row| row.pair_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for selected in selected_rows {
        if !row_pair_ids.contains(selected.pair_id.as_str()) {
            failures.push(format!(
                "{} selected pair missing from candidate universe",
                selected.pair_id
            ));
        }
    }
    if rows
        .iter()
        .any(|row| row.cutline_status == "ranked-above-cutline-but-unselected")
    {
        failures.push("unselected candidate ranked above selected budget cutline".to_string());
    }
    for row in rows {
        if row.pair_id.trim().is_empty()
            || row.origin_id.trim().is_empty()
            || row.dest_id.trim().is_empty()
            || row.market_class.trim().is_empty()
            || row.required_routes.trim().is_empty()
            || row.required_stops.trim().is_empty()
            || row.evidence_basis.trim().is_empty()
            || row.cutline_status.trim().is_empty()
            || row.cutline_reason.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete candidate-pair row", row.pair_id));
        }
        if !matches!(row.target_hours.round() as u16, 36 | 48) {
            failures.push(format!(
                "{} has unsupported T1 target_hours {}",
                row.pair_id, row.target_hours
            ));
        }
        if !row.portfolio_selected
            && row.cutline_status == "dropped-at-cutline"
            && row.covered_by_selected_pair.trim().is_empty()
            && !row.cutline_reason.contains("source")
            && !row.cutline_reason.contains("budget")
            && !row.cutline_reason.contains("lower-tier")
        {
            failures.push(format!(
                "{} dropped without coverage, source, budget, or lower-tier reason",
                row.pair_id
            ));
        }
    }
    failures
}

