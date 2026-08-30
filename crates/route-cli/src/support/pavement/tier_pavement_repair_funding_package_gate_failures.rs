//! Helper `tier_pavement_repair_funding_package_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_repair_funding_package_gate_failures(
    rows: &[TierPavementRepairFundingPackageRow],
    disposition_rows: &[TierPavementRepairDispositionRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = disposition_rows
        .iter()
        .filter(|row| {
            row.disposition == "repair-funding-required"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .count();
    if expected > 0 && rows.len() != expected {
        failures.push(format!(
            "funding package rows {} do not match disposition rows {}",
            rows.len(),
            expected
        ));
    }
    for row in rows {
        if row.funding_package_id.trim().is_empty()
            || row.disposition_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.funding_package_status.trim().is_empty()
            || row.funding_commitment_status.trim().is_empty()
            || row.relief_eligibility.trim().is_empty()
            || row.blocked_claims.trim().is_empty()
            || row.next_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete funding row",
                row.state, row.route
            ));
        }
        if row.funding_package_status != "package-required" {
            failures.push(format!(
                "{} {} has unsupported package status {}",
                row.state, row.route, row.funding_package_status
            ));
        }
        if row.funding_commitment_status != "unfunded" {
            failures.push(format!(
                "{} {} has funding commitment before acceptance",
                row.state, row.route
            ));
        }
        if row.relief_eligibility != "not-eligible-for-relief" {
            failures.push(format!(
                "{} {} is relief eligible before funding",
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
                "{} {} lacks repair funding magnitude",
                row.state, row.route
            ));
        }
        if row.validation_status != "held" {
            failures.push(format!("{} {} is not held", row.state, row.route));
        }
    }
    failures
}
