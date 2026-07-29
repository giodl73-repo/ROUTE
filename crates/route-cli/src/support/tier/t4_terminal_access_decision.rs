//! Helper `t4_terminal_access_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_decision(
    row: &T3T4PressureIntakeRow,
    zone_id: &str,
) -> (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
) {
    if zone_id == "zone-assignment-needed" {
        return (
            "unassigned-local-access".to_string(),
            "assign local route to a T3 zone or terminal district".to_string(),
            "zone-assignment-needed".to_string(),
            "zone boundary plus terminal/local freight role".to_string(),
            "hide-until-assigned".to_string(),
            "T4 pressure lacks a deterministic zone assignment".to_string(),
            "data/t3-t4-access-gaps.csv".to_string(),
            "blocks promotion and sends the route to access-gap triage".to_string(),
            "review".to_string(),
        );
    }

    if row.current_score >= T3_THRESHOLD - 5.0 {
        let (terminal_obligation, evidence_required) = t4_terminal_source_contract(zone_id);
        (
            "terminal-upgrade-candidate".to_string(),
            terminal_obligation.to_string(),
            "terminal-review".to_string(),
            evidence_required.to_string(),
            "show-as-local-inset-candidate".to_string(),
            "within five points of T3 threshold but still a T4/local access problem".to_string(),
            "data/t3-t4-access-gaps.csv".to_string(),
            "holds local pressure as a terminal access candidate instead of selecting T3"
                .to_string(),
            "review".to_string(),
        )
    } else {
        (
            "local-access-column".to_string(),
            "maintain one-hour local access to a freight terminal or district".to_string(),
            "selected-local-access".to_string(),
            "terminal/local obligation can be documented inside zone inset".to_string(),
            "render-in-local-inset".to_string(),
            "below T3 promotion pressure and suitable for T4 local access treatment".to_string(),
            "data/t3-zone-map-diagnostics.csv".to_string(),
            "keeps local service in T4 while exposing it to map diagnostics".to_string(),
            "pass".to_string(),
        )
    }
}

