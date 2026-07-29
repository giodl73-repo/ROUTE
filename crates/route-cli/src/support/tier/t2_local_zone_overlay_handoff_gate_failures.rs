//! Helper `t2_local_zone_overlay_handoff_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_local_zone_overlay_handoff_gate_failures(
    rows: &[T2LocalZoneOverlayHandoffRow],
    docket_rows: &[T2ServiceClassRepairDocketRow],
) -> Vec<String> {
    let expected = docket_rows
        .iter()
        .filter(|row| row.service_repair_class == "local-zone")
        .map(|row| row.docket_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "local-zone handoff has {} rows but expected {} local-zone repair rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.handoff_id.trim().is_empty()
            || row.docket_id.trim().is_empty()
            || row.target_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.zone_role.trim().is_empty()
            || row.column_decision.trim().is_empty()
            || row.map_treatment.trim().is_empty()
            || row.handoff_decision.trim().is_empty()
            || row.handoff_reason.trim().is_empty()
            || row.blocks_claims.trim().is_empty()
            || row.required_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete local-zone handoff fields",
                row.route
            ));
        }
        if !seen.insert(row.docket_id.clone()) {
            failures.push(format!("{} appears more than once", row.docket_id));
        }
        if !expected.contains(row.docket_id.as_str()) {
            failures.push(format!("{} is not a local-zone repair row", row.docket_id));
        }
        if !row.handoff_decision.starts_with("held") {
            failures.push(format!("{} local-zone handoff was promoted", row.route));
        }
        if !row.qualification_effects.trim().is_empty() && !row.handoff_decision.starts_with("held")
        {
            failures.push(format!(
                "{} local-zone handoff carries qualification effects without hold",
                row.route
            ));
        }
        if row.blocks_claims != "game;incident;publication;upgrade" {
            failures.push(format!("{} does not preserve claim blockers", row.route));
        }
        if row.validation_status != "review" {
            failures.push(format!(
                "{} local-zone handoff must remain review",
                row.route
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from local-zone handoff"));
        }
    }
    failures
}

