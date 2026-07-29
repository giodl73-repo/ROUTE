//! Helper `bundle_architecture_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn bundle_architecture_gate_failures(rows: &[BundleArchitectureRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no bundle architecture rows emitted".to_string());
        return failures;
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.crate_name.trim().is_empty()
            || row.role.trim().is_empty()
            || row.bundle_entrypoint.trim().is_empty()
            || row.source_path.trim().is_empty()
            || row.required_tokens.trim().is_empty()
            || row.architecture_status.trim().is_empty()
            || row.next_action.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete architecture row",
                row.crate_name
            ));
        }
        if !seen.insert(row.crate_name.clone()) {
            failures.push(format!("{} has duplicate architecture row", row.crate_name));
        }
        if row.validation_status != "pass" {
            failures.push(format!(
                "{} bundle architecture check failed: {}",
                row.crate_name, row.next_action
            ));
        }
        if !matches!(
            row.architecture_status.as_str(),
            "bundle-native" | "bundle-upstream"
        ) {
            failures.push(format!(
                "{} has unknown architecture status {}",
                row.crate_name, row.architecture_status
            ));
        }
    }
    failures
}

