//! Helper `fletch_source_handoff_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn fletch_source_handoff_gate_failures(
    report: &route_data::FletchSourceHandoffReport,
) -> Vec<String> {
    let mut failures = Vec::new();
    if !report.registry_valid {
        failures.push(format!(
            "registry {} is not valid ({} findings)",
            report.registry_id, report.validation_finding_count
        ));
    }
    if !report.missing_policy_families.is_empty() {
        failures.push(format!(
            "missing FLETCH coverage for source policy families: {}",
            report.missing_policy_families.join(", ")
        ));
    }
    if report.rows.is_empty() {
        failures.push("FLETCH source handoff emitted no rows".to_string());
    }
    for row in &report.rows {
        if row.validation_status != "pass" {
            failures.push(format!("{} handoff row is not valid", row.fletch_id));
        }
        if row.fetch_family.trim().is_empty() {
            failures.push(format!("{} missing fetch_family metadata", row.fletch_id));
        }
        if row.cache_targets.trim().is_empty() {
            failures.push(format!("{} missing cache targets", row.fletch_id));
        }
        if row.activation_rule.trim().is_empty() {
            failures.push(format!("{} missing activation rule", row.fletch_id));
        }
        if row.route_validation_floor.trim().is_empty() {
            failures.push(format!("{} missing ROUTE validation floor", row.fletch_id));
        }
    }
    failures
}

