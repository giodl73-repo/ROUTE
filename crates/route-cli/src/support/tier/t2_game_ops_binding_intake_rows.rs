//! Helper `t2_game_ops_binding_intake_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_binding_intake_rows(
    budget_rows: &[OptimizerConstraintBudgetRow],
) -> Vec<T2GameOpsBindingIntakeRow> {
    let mut rows = budget_rows
        .iter()
        .filter(|row| {
            row.tier == "T2"
                && constraint_class_values(&row.top_constraint_classes)
                    .iter()
                    .any(|class| class == "game_ops_bundle_binding")
        })
        .map(|row| T2GameOpsBindingIntakeRow {
            intake_id: format!("T2GAMEOPSINTAKE-{}", stable_id_fragment(&row.budget_id)),
            budget_id: row.budget_id.clone(),
            subject_id: row.subject_id.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            route: row.route.clone(),
            claim_blocker_count: row.claim_blocker_count,
            blocked_claims: row.blocking_claims.clone(),
            top_constraint_classes: row.top_constraint_classes.clone(),
            qualification_effects: row.qualification_effects.clone(),
            next_artifacts: row.next_artifacts.clone(),
            constraint_ledger_artifact: row.constraint_ledger_artifact.clone(),
            intake_status: "decision-needed".to_string(),
            decision_artifact: "data/t2-game-ops-binding-decisions.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.subject_id.cmp(&right.subject_id))
    });
    rows
}
