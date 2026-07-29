//! Helper `lower_tier_score_pressure_row`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn lower_tier_score_pressure_row(
    row: &TierTableScoreRow,
    pressure_type: &str,
    witness_action: &str,
    target_tier: &str,
    selection_basis: &str,
) -> LowerTierPressureWitnessRow {
    LowerTierPressureWitnessRow {
        route: row.route.clone(),
        current_tier: row.tier.clone(),
        current_score: row.score,
        confidence: row.confidence,
        confidence_label: row.confidence_label.clone(),
        pressure_type: pressure_type.to_string(),
        witness_action: witness_action.to_string(),
        target_tier: target_tier.to_string(),
        selection_basis: selection_basis.to_string(),
        source_artifact: "data/tier-table.csv".to_string(),
        next_artifact: if target_tier == "T2" {
            "data/tier-contact-witnesses.csv".to_string()
        } else {
            "data/tier-region-workloads.csv".to_string()
        },
        validation_status: "review".to_string(),
    }
}

