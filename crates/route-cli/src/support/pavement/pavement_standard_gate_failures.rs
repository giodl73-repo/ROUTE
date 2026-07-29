//! Helper `pavement_standard_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pavement_standard_gate_failures(rows: &[PavementStandardRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("pavement standards ledger has no rows".to_string());
        return failures;
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        let tier = row.tier.trim();
        if !seen.insert(tier.to_string()) {
            failures.push(format!("{tier} duplicate pavement standard row"));
        }
        if !matches!(tier, "T1" | "T2" | "T3" | "T4") {
            failures.push(format!("{tier} invalid pavement tier"));
        }
        if row.road_role.trim().is_empty()
            || row.target_pavement_condition.trim().is_empty()
            || row.freight_ride_requirement.trim().is_empty()
            || row.transit_ride_requirement.trim().is_empty()
            || row.repair_trigger.trim().is_empty()
            || row.allowed_exception.trim().is_empty()
            || row.source_contract.trim().is_empty()
        {
            failures.push(format!("{tier} has incomplete pavement standard fields"));
        }
        if !row.max_iri_m_per_km.is_finite() || row.max_iri_m_per_km <= 0.0 {
            failures.push(format!(
                "{tier} has invalid max IRI {}",
                row.max_iri_m_per_km
            ));
        }
        if row.inspection_interval_months == 0 || row.inspection_interval_months > 36 {
            failures.push(format!(
                "{tier} has invalid inspection interval {}",
                row.inspection_interval_months
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{tier} has invalid validation status {}",
                row.validation_status
            ));
        }
    }
    for tier in ["T1", "T2", "T3", "T4"] {
        if !seen.contains(tier) {
            failures.push(format!("{tier} missing pavement standard row"));
        }
    }
    failures
}

