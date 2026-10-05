//! Helper `optimizer_constraint_budget_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_constraint_budget_rows(
    ledger_rows: &[OptimizerConstraintLedgerRow],
) -> Vec<OptimizerConstraintBudgetRow> {
    let mut builders =
        std::collections::BTreeMap::<String, OptimizerConstraintBudgetBuilder>::new();

    for row in ledger_rows {
        let (subject_scope, subject_id) = optimizer_constraint_budget_subject(row);
        let key = format!("{}|{}|{}", row.tier, subject_scope, subject_id);
        let builder = builders
            .entry(key)
            .or_insert_with(|| OptimizerConstraintBudgetBuilder {
                optimizer_run_id: row.optimizer_run_id.clone(),
                tier: row.tier.clone(),
                region_id: row.region_id.clone(),
                subject_scope: subject_scope.clone(),
                subject_id: subject_id.clone(),
                segment_bundle_id: if subject_scope == "bundle" {
                    row.segment_bundle_id.clone()
                } else {
                    String::new()
                },
                route: row.route.clone(),
                ..Default::default()
            });

        builder.ledger_row_count += 1;
        if row.behavior_type == "identity-blocker" || row.behavior_type == "selection-hard" {
            builder.hard_blocker_count += 1;
        }
        if row.behavior_type == "claim-blocker" {
            builder.claim_blocker_count += 1;
        }
        if row.validation_status == "review"
            || row.constraint_status == "review"
            || row.constraint_status == "held"
        {
            builder.review_count += 1;
        }
        if row.behavior_type == "budget-debt" {
            builder.budget_debt_count += 1;
        }
        builder.constraint_debt_cost_m =
            round_cost_m(builder.constraint_debt_cost_m + row.budget_cost_m);
        if row.cost_category == "lifecycle" || row.cost_category == "maintenance" {
            builder.lifecycle_debt_cost_m =
                round_cost_m(builder.lifecycle_debt_cost_m + row.budget_cost_m);
        }
        builder.constraint_penalty_score =
            round_cost_m(builder.constraint_penalty_score + row.penalty_score);
        *builder
            .class_counts
            .entry(row.constraint_class.clone())
            .or_default() += 1;
        for claim in row.blocks_claims.split('|').map(str::trim) {
            if !claim.is_empty() {
                builder.blocking_claims.insert(claim.to_string());
            }
        }
        if !row.next_artifact.trim().is_empty() {
            builder.next_artifacts.insert(row.next_artifact.clone());
        }
        insert_optimizer_qualification_effects(
            &mut builder.qualification_effects,
            &row.optimizer_effect,
        );
        if builder.route.is_empty() && !row.route.is_empty() {
            builder.route = row.route.clone();
        }
    }

    builders
        .into_values()
        .map(|builder| {
            let top_constraint_classes = top_constraint_classes(&builder.class_counts);
            let validation_status = if builder.hard_blocker_count > 0 {
                "blocked"
            } else if builder.claim_blocker_count > 0 || builder.review_count > 0 {
                "review"
            } else {
                "pass"
            };
            OptimizerConstraintBudgetRow {
                budget_id: format!(
                    "CB-{}-{}-{}",
                    builder.tier,
                    builder.subject_scope.to_ascii_uppercase(),
                    stable_id_fragment(&builder.subject_id)
                ),
                optimizer_run_id: builder.optimizer_run_id,
                tier: builder.tier,
                region_id: builder.region_id,
                subject_scope: builder.subject_scope,
                subject_id: builder.subject_id,
                segment_bundle_id: builder.segment_bundle_id,
                route: builder.route,
                ledger_row_count: builder.ledger_row_count,
                hard_blocker_count: builder.hard_blocker_count,
                claim_blocker_count: builder.claim_blocker_count,
                review_count: builder.review_count,
                budget_debt_count: builder.budget_debt_count,
                constraint_debt_cost_m: builder.constraint_debt_cost_m,
                lifecycle_debt_cost_m: builder.lifecycle_debt_cost_m,
                constraint_penalty_score: builder.constraint_penalty_score,
                top_constraint_classes,
                blocking_claims: join_string_set(&builder.blocking_claims),
                qualification_effects: join_pipe_set(&builder.qualification_effects),
                next_artifacts: join_string_set(&builder.next_artifacts),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                validation_status: validation_status.to_string(),
            }
        })
        .collect()
}
