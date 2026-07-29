//! Helper `t4_terminal_contact_source_catalog_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_source_catalog_gate_failures(
    rows: &[T4TerminalContactSourceCatalogRow],
    plan_rows: &[T4TerminalContactSourcePlanRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = plan_rows.iter().fold(
        std::collections::BTreeMap::<String, usize>::new(),
        |mut counts, row| {
            *counts.entry(row.terminal_district.clone()).or_default() += 1;
            counts
        },
    );

    if rows.is_empty() {
        failures.push("no terminal district source catalog rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "source catalog has {} districts but expected {}",
            rows.len(),
            expected.len()
        ));
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.catalog_id.trim().is_empty()
            || row.terminal_district.trim().is_empty()
            || row.source_family.trim().is_empty()
            || row.source_access_mode.trim().is_empty()
            || row.required_proof_fields.trim().is_empty()
            || row.acquisition_status.trim().is_empty()
            || row.proof_blocker.trim().is_empty()
            || row.cache_policy_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete source catalog fields",
                row.catalog_id
            ));
        }
        if !seen.insert(row.terminal_district.clone()) {
            failures.push(format!(
                "{} appears more than once in source catalog",
                row.terminal_district
            ));
        }
        match expected.get(&row.terminal_district) {
            Some(expected_count) if *expected_count == row.route_task_count => {}
            Some(expected_count) => failures.push(format!(
                "{} has {} route tasks but expected {}",
                row.terminal_district, row.route_task_count, expected_count
            )),
            None => failures.push(format!(
                "{} does not appear in the route source plan",
                row.terminal_district
            )),
        }
        if !row
            .required_proof_fields
            .contains("route-to-terminal contact statement")
        {
            failures.push(format!(
                "{} lacks route-to-terminal contact proof fields",
                row.terminal_district
            ));
        }
        if !matches!(
            row.acquisition_status.as_str(),
            "planned" | "source-needed" | "source-backed" | "blocked"
        ) {
            failures.push(format!(
                "{} has invalid acquisition status {}",
                row.terminal_district, row.acquisition_status
            ));
        }
        if row.acquisition_status == "source-needed"
            && (!row.source_access_mode.contains("source-needed")
                || row.validation_status != "review")
        {
            failures.push(format!(
                "{} source-needed catalog row must keep source-needed access mode and review status",
                row.terminal_district
            ));
        }
        if row.acquisition_status == "source-needed"
            && !row.proof_blocker.contains("no safe live fetcher")
        {
            failures.push(format!(
                "{} source-needed catalog row lacks fetch/cache blocker",
                row.terminal_district
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review" | "held") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.terminal_district, row.validation_status
            ));
        }
    }

    for district in expected.keys() {
        if !seen.contains(district) {
            failures.push(format!("{district} is missing from source catalog"));
        }
    }

    failures
}

