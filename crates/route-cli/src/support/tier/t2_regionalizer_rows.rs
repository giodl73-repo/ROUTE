//! Helper `t2_regionalizer_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_regionalizer_rows(rows: &[TierCandidateColumnRow]) -> Vec<T2RegionalizerRow> {
    rows.iter()
        .filter(|row| row.tier.eq_ignore_ascii_case("T2"))
        .filter(|row| matches!(row.column_decision.as_str(), "selected" | "review"))
        .map(|row| {
            let treatment_status = if row.column_decision == "selected" {
                "selected-treatment"
            } else {
                "review-treatment"
            };
            T2RegionalizerRow {
                tier: row.tier.clone(),
                region_id: format!("component-{}", row.component_id),
                component_id: row.component_id,
                route: row.route.clone(),
                parent_trunks: row.parent_trunks.clone(),
                route_miles: row.route_miles,
                column_decision: row.column_decision.clone(),
                treatment_status: treatment_status.to_string(),
                evidence_status: row.evidence_status.clone(),
                pavement_debt_cost_m: row.pavement_debt_cost_m,
                pavement_debt_class: row.pavement_debt_class.clone(),
                pavement_debt_basis: row.pavement_debt_basis.clone(),
                hard_blocker_count: row.hard_blocker_count,
                claim_blocker_count: row.claim_blocker_count,
                constraint_debt_cost_m: row.constraint_debt_cost_m,
                lifecycle_debt_cost_m: row.lifecycle_debt_cost_m,
                constraint_penalty_score: row.constraint_penalty_score,
                top_constraint_classes: row.top_constraint_classes.clone(),
                qualification_effects: row.qualification_effects.clone(),
                constraint_ledger_artifact: row.constraint_ledger_artifact.clone(),
                regionalizer_action: if row.column_decision == "selected" {
                    "include-in-regional-treatment"
                } else {
                    "hold-for-parent-region-review"
                }
                .to_string(),
                validation_status: if row.column_decision == "selected" {
                    "pass"
                } else {
                    "review"
                }
                .to_string(),
            }
        })
        .collect()
}

