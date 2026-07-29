//! Helper `optimizer_constraint_ledger_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_constraint_ledger_gate_failures(rows: &[OptimizerConstraintLedgerRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no optimizer constraint ledger rows emitted".to_string());
        return failures;
    }

    let mut ids = std::collections::BTreeSet::new();
    for row in rows {
        if !ids.insert(row.constraint_id.as_str()) {
            failures.push(format!("duplicate constraint id {}", row.constraint_id));
        }
        if row.constraint_id.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.constraint_class.trim().is_empty()
            || row.behavior_type.trim().is_empty()
            || row.constraint_scope.trim().is_empty()
            || row.subject_id.trim().is_empty()
            || row.source_artifact.trim().is_empty()
            || row.source_row_id.trim().is_empty()
            || row.standard_artifact.trim().is_empty()
            || row.evidence_status.trim().is_empty()
            || row.constraint_status.trim().is_empty()
            || row.repair_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete required fields",
                row.constraint_id
            ));
        }
        if matches!(
            row.constraint_scope.as_str(),
            "bundle" | "segment" | "service" | "corridor"
        ) && row.segment_bundle_id.trim().is_empty()
            && row.national_segment_id.trim().is_empty()
        {
            failures.push(format!(
                "{} is segment-bearing but lacks bundle/member identity",
                row.constraint_id
            ));
        }
        if row.behavior_type == "budget-debt" {
            if row.budget_cost_m <= 0.0
                || row.cost_category.trim().is_empty()
                || row.cost_basis.trim().is_empty()
                || row.cost_confidence.trim().is_empty()
                || row.payment_action.trim().is_empty()
            {
                failures.push(format!(
                    "{} has incomplete budget-debt contract",
                    row.constraint_id
                ));
            }
        }
        if row.behavior_type == "claim-blocker" && row.blocks_claims.trim().is_empty() {
            failures.push(format!(
                "{} does not name blocked claims",
                row.constraint_id
            ));
        }
        if row.behavior_type == "selection-hard"
            && (row.exception_id.trim().is_empty() || row.exception_artifact.trim().is_empty())
        {
            failures.push(format!(
                "{} selection-hard row lacks exception lineage",
                row.constraint_id
            ));
        }
        if !row.observed_value.trim().is_empty()
            && (row.threshold_value.trim().is_empty() || row.measurement_unit.trim().is_empty())
        {
            failures.push(format!(
                "{} has observed value without threshold/unit",
                row.constraint_id
            ));
        }
    }
    failures
}

