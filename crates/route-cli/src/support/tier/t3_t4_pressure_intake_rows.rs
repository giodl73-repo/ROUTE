//! Helper `t3_t4_pressure_intake_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_t4_pressure_intake_rows(
    pressure_rows: &[LowerTierPressureWitnessRow],
) -> Vec<T3T4PressureIntakeRow> {
    let mut rows = pressure_rows
        .iter()
        .map(|row| {
            let (intake_class, intake_action, target_tier, next_artifact, optimizer_effect) =
                t3_t4_pressure_intake_decision(row);
            T3T4PressureIntakeRow {
                route: row.route.clone(),
                source_pressure_type: row.pressure_type.clone(),
                current_tier: row.current_tier.clone(),
                current_score: row.current_score,
                target_tier: target_tier.to_string(),
                intake_class: intake_class.to_string(),
                intake_action: intake_action.to_string(),
                selection_basis: row.selection_basis.clone(),
                source_artifact: row.source_artifact.clone(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        a.intake_class
            .cmp(&b.intake_class)
            .then_with(|| b.current_score.total_cmp(&a.current_score))
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}
