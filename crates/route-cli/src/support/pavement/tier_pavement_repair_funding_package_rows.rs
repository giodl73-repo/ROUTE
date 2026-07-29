//! Helper `tier_pavement_repair_funding_package_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_repair_funding_package_rows(
    disposition_rows: &[TierPavementRepairDispositionRow],
) -> Vec<TierPavementRepairFundingPackageRow> {
    disposition_rows
        .iter()
        .filter(|row| {
            row.disposition == "repair-funding-required"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementRepairFundingPackageRow {
            funding_package_id: format!(
                "PAVEMENTREPAIRFUNDING-{}",
                stable_id_fragment(&row.disposition_id)
            ),
            disposition_id: row.disposition_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            blocked_member_count: row.blocked_member_count,
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            funding_package_status: "package-required".to_string(),
            funding_commitment_status: "unfunded".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action:
                "attach accepted funding commitment or choose downgrade/exclusion before relief replay"
                    .to_string(),
            next_artifact: "data/tier-pavement-repair-funding-package.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

