//! Helper `t3_t4_access_gap_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_t4_access_gap_gate_failures(rows: &[T3T4AccessGapRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T3/T4 access gaps emitted".to_string());
        return failures;
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.gap_id.trim().is_empty()
            || row.source_surface.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.promise_horizon_hours == 0
            || row.gap_class.trim().is_empty()
            || row.gap_reason.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.repair_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!("{} has incomplete access gap fields", row.gap_id));
        }
        if !seen.insert(row.gap_id.clone()) {
            failures.push(format!("{} is duplicated", row.gap_id));
        }
        if row.next_artifact == "data/t3-t4-access-gaps.csv" {
            failures.push(format!(
                "{} loops back to the access gap artifact",
                row.gap_id
            ));
        }
        if row.upward_pressure_allowed {
            failures.push(format!(
                "{} allows upward pressure without higher-tier proof",
                row.gap_id
            ));
        }
        if row.gap_class == "zone-assignment-needed" && row.zone_id != "zone-assignment-needed" {
            failures.push(format!(
                "{} has zone-assignment-needed class but zone {}",
                row.gap_id, row.zone_id
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.gap_id, row.validation_status
            ));
        }
    }
    failures
}
