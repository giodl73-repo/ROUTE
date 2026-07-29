//! Helper `t1_sla_candidate_pair_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_sla_candidate_pair_rows(
    candidate_rows: &[T1SlaCandidateUniverseRow],
    selected_rows: &[T1SlaPairRow],
    selected_budget: usize,
) -> Vec<T1SlaCandidatePairRow> {
    let selected_pair_ids = selected_rows
        .iter()
        .map(|row| row.pair_id.trim().to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let mut rows = candidate_rows
        .iter()
        .map(|row| {
            let total_score = t1_sla_candidate_pair_score(row);
            let portfolio_selected = selected_pair_ids.contains(row.pair_id.trim());
            T1SlaCandidatePairRow {
                rank: 0,
                pair_id: row.pair_id.clone(),
                origin_id: row.origin_id.clone(),
                dest_id: row.dest_id.clone(),
                target_hours: row.target_hours,
                market_class: row.market_class.clone(),
                total_score,
                market_score: row.market_score,
                conversion_score: row.conversion_score,
                coverage_score: row.coverage_score,
                reuse_score: row.reuse_score,
                resilience_score: row.resilience_score,
                evidence_score: row.evidence_score,
                budget_penalty: row.budget_penalty,
                portfolio_selected,
                selected_budget,
                cutline_status: String::new(),
                cutline_reason: String::new(),
                covered_by_selected_pair: row.covered_by_selected_pair.clone(),
                required_routes: row.required_routes.clone(),
                required_stops: row.required_stops.clone(),
                evidence_basis: row.evidence_basis.clone(),
                validation_status: String::new(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        b.total_score
            .total_cmp(&a.total_score)
            .then_with(|| b.portfolio_selected.cmp(&a.portfolio_selected))
            .then_with(|| a.pair_id.cmp(&b.pair_id))
    });

    for (idx, row) in rows.iter_mut().enumerate() {
        row.rank = idx + 1;
        if row.portfolio_selected {
            row.cutline_status = "selected-portfolio".to_string();
            row.cutline_reason = "inside-selected-t1-promise-budget".to_string();
            row.validation_status = "pass".to_string();
        } else if row.rank <= selected_budget {
            row.cutline_status = "ranked-above-cutline-but-unselected".to_string();
            row.cutline_reason = "candidate-score-conflicts-with-selected-portfolio".to_string();
            row.validation_status = "review".to_string();
        } else {
            row.cutline_status = "dropped-at-cutline".to_string();
            row.cutline_reason = candidate_rows
                .iter()
                .find(|candidate| candidate.pair_id == row.pair_id)
                .map(|candidate| candidate.drop_reason_hint.trim())
                .filter(|hint| !hint.is_empty())
                .unwrap_or("below-selected-budget-cutline")
                .to_string();
            row.validation_status = "pass".to_string();
        }
    }
    rows
}

