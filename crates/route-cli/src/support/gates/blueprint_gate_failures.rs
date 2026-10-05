//! Helper `blueprint_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn blueprint_gate_failures(rows: &[BlueprintPackageRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["blueprint package ledger has no rows".to_string()];
    }

    let mut failures = Vec::new();
    for row in rows {
        if let Some(failure) = blueprint_row_contract_failure(row) {
            failures.push(failure);
        }
        if row
            .stakeholder_class
            .trim()
            .eq_ignore_ascii_case("conditional_expansion")
        {
            for (field, value) in [
                ("mitigation_companion", row.mitigation_companion.as_str()),
                ("row_complexity", row.row_complexity.as_str()),
                ("maintenance_burden", row.maintenance_burden.as_str()),
                (
                    "community_exposure_check",
                    row.community_exposure_check.as_str(),
                ),
            ] {
                if blueprint_field_is_not_applicable(value) {
                    failures.push(format!(
                        "{} conditional expansion lacks required {}",
                        row.package_id, field
                    ));
                }
            }
        }
        if row
            .stakeholder_class
            .trim()
            .eq_ignore_ascii_case("source_gated_must_have")
            && row.rural_access_exception.trim().is_empty()
        {
            failures.push(format!(
                "{} source-gated package lacks rural_access_exception field",
                row.package_id
            ));
        }
    }
    failures
}
