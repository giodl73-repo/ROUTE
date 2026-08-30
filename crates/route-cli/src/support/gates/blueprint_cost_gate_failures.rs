//! Helper `blueprint_cost_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn blueprint_cost_gate_failures(
    rows: &[BlueprintCostRow],
    packages: &[BlueprintPackageRow],
) -> Vec<String> {
    if rows.is_empty() {
        return vec!["blueprint cost ledger has no rows".to_string()];
    }

    let package_ids = packages
        .iter()
        .map(|row| row.package_id.as_str())
        .collect::<std::collections::HashSet<_>>();
    let mut failures = Vec::new();
    for row in rows {
        if let Some(failure) = blueprint_cost_row_failure(row, &package_ids) {
            failures.push(failure);
        }
    }

    for package in packages {
        if !rows
            .iter()
            .any(|row| row.package_id.trim() == package.package_id.trim())
        {
            failures.push(format!("{} missing cost range row", package.package_id));
        }
    }

    failures
}
