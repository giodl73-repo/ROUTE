//! Helper `tier_pavement_repair_disposition_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_repair_disposition_gate_failures(
    rows: &[TierPavementRepairDispositionRow],
    repair_rows: &[TierPavementRepairDebtReviewRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = repair_rows
        .iter()
        .filter(|row| {
            row.source_priority == "A"
                && row.repair_debt_status == "confirmed-repair-debt"
                && row.validation_status == "review"
        })
        .count();
    if expected > 0 && rows.len() != expected {
        failures.push(format!(
            "repair disposition rows {} do not match repair review rows {}",
            rows.len(),
            expected
        ));
    }
    for row in rows {
        if row.disposition_id.trim().is_empty()
            || row.repair_review_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.source_priority.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete disposition row",
                row.state, row.route
            ));
        }
        if row.disposition != "repair-funding-required" {
            failures.push(format!(
                "{} {} has unsupported disposition {}",
                row.state, row.route, row.disposition
            ));
        }
        if row.relief_eligibility != "not-eligible-for-relief" {
            failures.push(format!(
                "{} {} is relief eligible before repair",
                row.state, row.route
            ));
        }
        if row.claim_blocker_delta != 0 {
            failures.push(format!(
                "{} {} reduces blockers before relief",
                row.state, row.route
            ));
        }
        if row.blocked_member_count == 0 || row.estimated_repair_cost_m <= 0.0 {
            failures.push(format!(
                "{} {} lacks repair debt magnitude",
                row.state, row.route
            ));
        }
        if row.validation_status != "held" {
            failures.push(format!("{} {} is not held", row.state, row.route));
        }
    }
    failures
}

