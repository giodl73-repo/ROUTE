//! Extracted helper `t1_line_selector_rows` from main.
use super::*;

pub(crate) fn t1_line_selector_rows(
    tier_table: &Path,
    stop_candidates: &Path,
    sla_pairs: &Path,
    score_exceptions: &Path,
    constraint_budget: &Path,
    route_budget: usize,
    city_budget: usize,
    stop_budget: usize,
) -> Result<Vec<T1LineSelectorRow>> {
    let mut tier_rows = csv::Reader::from_path(tier_table)
        .with_context(|| format!("reading {}", tier_table.display()))?
        .deserialize::<TierTableInputRow>()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("parsing {}", tier_table.display()))?;
    let sla_rows = csv::Reader::from_path(sla_pairs)
        .with_context(|| format!("reading {}", sla_pairs.display()))?
        .deserialize::<T1SlaPairRow>()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("parsing {}", sla_pairs.display()))?;
    let score_exception_rows = load_t1_score_exceptions(score_exceptions)
        .with_context(|| format!("loading {}", score_exceptions.display()))?;
    let constraint_budget_rows = load_optimizer_constraint_budget(constraint_budget)
        .with_context(|| format!("loading {}", constraint_budget.display()))?;
    let constraint_budget_index = optimizer_constraint_budget_index(&constraint_budget_rows);
    let demoted_score_routes = score_exception_rows
        .iter()
        .filter(|row| matches!(row.decision.trim(), "demote" | "replace"))
        .map(|row| normalise_designation(&row.route))
        .collect::<std::collections::BTreeSet<_>>();
    let score_exception_decisions = score_exception_rows
        .iter()
        .map(|row| {
            (
                normalise_designation(&row.route),
                row.decision.trim().to_string(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut required_route_pairs = std::collections::BTreeMap::<String, Vec<String>>::new();
    let mut required_route_priority = std::collections::BTreeMap::<String, u8>::new();
    for pair in &sla_rows {
        if pair.origin_id.trim().is_empty()
            || pair.dest_id.trim().is_empty()
            || pair.market_class.trim().is_empty()
            || pair.required_stops.trim().is_empty()
            || pair.evidence_basis.trim().is_empty()
        {
            anyhow::bail!("{} has incomplete SLA pair contract", pair.pair_id);
        }
        if !matches!(pair.target_hours.round() as u16, 36 | 48) {
            anyhow::bail!(
                "{} has unsupported T1 target_hours {}; T1 promises must be 36h or 48h",
                pair.pair_id,
                pair.target_hours
            );
        }
        for route in pair.required_routes.split(';') {
            let route = normalise_designation(route.trim());
            if !route.is_empty() {
                required_route_pairs
                    .entry(route.clone())
                    .or_default()
                    .push(pair.pair_id.clone());
                required_route_priority
                    .entry(route)
                    .and_modify(|priority| *priority = (*priority).max(pair.priority))
                    .or_insert(pair.priority);
            }
        }
    }
    tier_rows.sort_by(|a, b| {
        let a_route = normalise_designation(&a.route);
        let b_route = normalise_designation(&b.route);
        let (_, _, _, _, a_penalty, _, _, _) =
            constraint_budget_for_candidate(&a_route, "", &constraint_budget_index);
        let (_, _, _, _, b_penalty, _, _, _) =
            constraint_budget_for_candidate(&b_route, "", &constraint_budget_index);
        let a_adjusted_score = a.score - a_penalty;
        let b_adjusted_score = b.score - b_penalty;
        required_route_priority
            .get(&b_route)
            .unwrap_or(&0)
            .cmp(required_route_priority.get(&a_route).unwrap_or(&0))
            .then_with(|| b_adjusted_score.total_cmp(&a_adjusted_score))
            .then_with(|| b.score.total_cmp(&a.score))
            .then_with(|| a_route.cmp(&b_route))
    });

    let mut stop_rows = csv::Reader::from_path(stop_candidates)
        .with_context(|| format!("reading {}", stop_candidates.display()))?
        .deserialize::<StopCandidateRow>()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("parsing {}", stop_candidates.display()))?;
    stop_rows.sort_by(|a, b| {
        stop_candidate_selector_score(b)
            .cmp(&stop_candidate_selector_score(a))
            .then_with(|| a.name.cmp(&b.name))
    });
    let top_city_ids = stop_rows
        .iter()
        .take(city_budget)
        .map(|row| row.stop_id.clone())
        .collect::<std::collections::BTreeSet<_>>();

    let mut remaining_stop_budget = stop_budget;
    let mut selected_routes = 0usize;
    let mut rows = Vec::new();
    for (idx, row) in tier_rows.iter().enumerate() {
        let route = normalise_designation(&row.route);
        let route_stops = stop_rows
            .iter()
            .filter(|stop| {
                stop_candidate_routes(stop)
                    .iter()
                    .any(|item| item == &route)
            })
            .collect::<Vec<_>>();
        let top_city_stops = route_stops
            .iter()
            .filter(|stop| top_city_ids.contains(&stop.stop_id))
            .map(|stop| stop.stop_id.as_str())
            .collect::<Vec<_>>();
        let route_sla_pairs = required_route_pairs
            .get(&route)
            .cloned()
            .unwrap_or_default();
        let sla_pair_count = route_sla_pairs.len();
        let is_t1 = row.tier.trim().eq_ignore_ascii_case("T1");
        let score_exception_demoted = sla_pair_count == 0 && demoted_score_routes.contains(&route);
        let score_exception_decision = if sla_pair_count == 0 {
            score_exception_decisions.get(&route).map(String::as_str)
        } else {
            None
        };
        let score_exception_kept =
            matches!(score_exception_decision, Some("keep" | "conditional_keep"));
        let (
            hard_blocker_count,
            claim_blocker_count,
            constraint_debt_cost_m,
            lifecycle_debt_cost_m,
            constraint_penalty_score,
            top_constraint_classes,
            _qualification_effects,
            constraint_ledger_artifact,
        ) = constraint_budget_for_candidate(&route, "", &constraint_budget_index);
        let constraint_adjusted_score = row.score - constraint_penalty_score;
        let has_budget =
            selected_routes < route_budget && route_stops.len() <= remaining_stop_budget;
        let selected = (sla_pair_count > 0 || (is_t1 && score_exception_kept)) && has_budget;
        let decision = if selected {
            "select"
        } else if sla_pair_count > 0 {
            "reject-sla-budget"
        } else if score_exception_demoted {
            "reject-score-exception"
        } else if !is_t1 {
            "reject-tier"
        } else if selected_routes >= route_budget {
            "reject-route-budget"
        } else {
            "reject-stop-budget"
        };
        let reason = if selected {
            if sla_pair_count > 0 {
                "sla-required-budget-fit"
            } else if score_exception_decision == Some("keep") {
                "score-exception-keep"
            } else if score_exception_kept {
                "score-exception-conditional-keep"
            } else {
                "score-ranked-budget-fit"
            }
        } else if sla_pair_count > 0 {
            "sla-required-budget-exhausted"
        } else if score_exception_demoted {
            "score-exception-demoted"
        } else if !is_t1 {
            "not-t1-score-band"
        } else if selected_routes >= route_budget {
            "route-budget-exhausted"
        } else {
            "stop-budget-exhausted"
        };
        if selected {
            selected_routes += 1;
            remaining_stop_budget = remaining_stop_budget.saturating_sub(route_stops.len());
        }
        rows.push(T1LineSelectorRow {
            route,
            tier: row.tier.clone(),
            score: row.score,
            constraint_adjusted_score,
            rank: idx + 1,
            selected,
            selected_stop_count: route_stops.len(),
            top_city_stop_count: top_city_stops.len(),
            sla_pair_count,
            budget_cost: route_stops.len(),
            hard_blocker_count,
            claim_blocker_count,
            constraint_debt_cost_m,
            lifecycle_debt_cost_m,
            constraint_penalty_score,
            top_constraint_classes,
            constraint_ledger_artifact,
            decision,
            reason,
            selected_stops: route_stops
                .iter()
                .map(|stop| stop.stop_id.as_str())
                .collect::<Vec<_>>()
                .join(";"),
            top_city_stops: top_city_stops.join(";"),
            sla_pairs: route_sla_pairs.join(";"),
        });
    }
    Ok(rows)
}

