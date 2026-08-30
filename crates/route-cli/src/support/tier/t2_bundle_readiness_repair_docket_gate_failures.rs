//! Helper `t2_bundle_readiness_repair_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_readiness_repair_docket_gate_failures(
    rows: &[T2BundleReadinessRepairDocketRow],
    readiness_rows: &[T2BundleReadinessDispositionRow],
) -> Vec<String> {
    let expected = readiness_rows
        .iter()
        .filter(|row| row.disposition == "repair-needed")
        .map(|row| row.disposition_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "readiness repair docket has {} rows but expected {} repair-needed readiness rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.repair_id.trim().is_empty()
            || row.disposition_id.trim().is_empty()
            || row.target_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.readiness_class.trim().is_empty()
            || row.repair_decision.trim().is_empty()
            || row.repair_action.trim().is_empty()
            || row.required_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.blocks_claims.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete readiness repair fields",
                row.route
            ));
        }
        if !seen.insert(row.disposition_id.clone()) {
            failures.push(format!("{} appears more than once", row.disposition_id));
        }
        if !expected.contains(row.disposition_id.as_str()) {
            failures.push(format!(
                "{} is not a repair-needed readiness row",
                row.disposition_id
            ));
        }
        if row.repair_decision != "repair-needed" {
            failures.push(format!("{} readiness repair was promoted", row.route));
        }
        if !row.qualification_effects.trim().is_empty() && row.repair_decision != "repair-needed" {
            failures.push(format!(
                "{} readiness repair carries qualification effects without repair decision",
                row.route
            ));
        }
        if row.blocks_claims != "game;incident;publication;upgrade" {
            failures.push(format!("{} does not preserve claim blockers", row.route));
        }
        if row.validation_status != "review" {
            failures.push(format!("{} readiness repair must remain review", row.route));
        }
        if !matches!(
            row.readiness_class.as_str(),
            "stop-chain" | "stitched-member" | "terminal-stop"
        ) {
            failures.push(format!(
                "{} has unsupported readiness class {}",
                row.route, row.readiness_class
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} missing from readiness repair docket"
            ));
        }
    }
    failures
}
