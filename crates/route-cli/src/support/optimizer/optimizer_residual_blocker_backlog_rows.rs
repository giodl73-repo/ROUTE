//! Helper `optimizer_residual_blocker_backlog_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_residual_blocker_backlog_rows(
    budget_rows: &[OptimizerConstraintBudgetRow],
) -> Vec<OptimizerResidualBlockerBacklogRow> {
    let mut builders = std::collections::BTreeMap::<String, ResidualBacklogBuilder>::new();
    for row in budget_rows
        .iter()
        .filter(|row| row.validation_status != "pass")
    {
        let (priority_class, blocker_family, next_wave) = optimizer_backlog_family(row);
        let key = format!("{priority_class}|{blocker_family}|{}", row.tier);
        let builder = builders
            .entry(key)
            .or_insert_with(|| ResidualBacklogBuilder {
                priority_class,
                blocker_family,
                tier: row.tier.clone(),
                next_wave,
                ..Default::default()
            });
        builder.subject_ids.insert(row.subject_id.clone());
        if !row.route.trim().is_empty() {
            builder.routes.insert(row.route.clone());
        }
        builder.total_hard_blockers += row.hard_blocker_count;
        builder.total_claim_blockers += row.claim_blocker_count;
        builder.total_budget_debt_count += row.budget_debt_count;
        builder.total_constraint_debt_cost_m =
            round_cost_m(builder.total_constraint_debt_cost_m + row.constraint_debt_cost_m);
        builder.total_constraint_penalty_score =
            round_cost_m(builder.total_constraint_penalty_score + row.constraint_penalty_score);
        for claim in row.blocking_claims.split(';').map(str::trim) {
            if !claim.is_empty() {
                builder.blocked_claims.insert(claim.to_string());
            }
        }
        for artifact in row.next_artifacts.split(['|', ';']).map(str::trim) {
            if !artifact.is_empty() {
                builder.next_artifacts.insert(artifact.to_string());
            }
        }
    }

    let mut rows = builders
        .into_values()
        .map(|builder| {
            let subject_count = builder.subject_ids.len();
            let route_count = builder.routes.len();
            OptimizerResidualBlockerBacklogRow {
                backlog_id: format!(
                    "ORB-{}-{}-{}",
                    builder.priority_class,
                    builder.tier,
                    stable_id_fragment(&builder.blocker_family)
                ),
                priority_class: builder.priority_class,
                blocker_family: builder.blocker_family,
                tier: builder.tier,
                blocked_claims: join_string_set(&builder.blocked_claims),
                subject_count,
                route_count,
                total_hard_blockers: builder.total_hard_blockers,
                total_claim_blockers: builder.total_claim_blockers,
                total_budget_debt_count: builder.total_budget_debt_count,
                total_constraint_debt_cost_m: builder.total_constraint_debt_cost_m,
                total_constraint_penalty_score: builder.total_constraint_penalty_score,
                representative_routes: join_limited_set(&builder.routes, 12),
                representative_subjects: join_limited_set(&builder.subject_ids, 12),
                next_artifacts: join_string_set(&builder.next_artifacts),
                backlog_decision: "triage-only-no-blocker-relief".to_string(),
                next_wave: builder.next_wave,
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.priority_class
            .cmp(&right.priority_class)
            .then_with(|| right.total_hard_blockers.cmp(&left.total_hard_blockers))
            .then_with(|| right.total_claim_blockers.cmp(&left.total_claim_blockers))
            .then_with(|| {
                right
                    .total_constraint_penalty_score
                    .partial_cmp(&left.total_constraint_penalty_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| left.blocker_family.cmp(&right.blocker_family))
    });
    rows
}
