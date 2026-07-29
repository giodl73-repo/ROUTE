//! Helper `stop_sla_promotion_row`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_sla_promotion_row(row: &StopSlaCandidateDocketRow) -> StopCandidateRow {
    let class = row
        .candidate_class
        .trim()
        .trim_end_matches('?')
        .to_ascii_uppercase();
    let evidence_status = if row.candidate_source_type == "stop-ledger" {
        row.candidate_evidence_status.clone()
    } else {
        "source_needed".to_string()
    };
    let source_type = row.candidate_source_type.replace('_', "-");
    let stop_role = if row.intersection_route_count >= 2 {
        format!("sla_spacing_candidate; route_contact; {source_type}")
    } else {
        format!("sla_spacing_candidate; {source_type}")
    };
    let next_step = format!(
        "Validate real interchange/service-city candidate for {} ({:.0} mi gap, {:.0} mi modeled gain); basis: {}",
        row.gap_segment, row.gap_miles, row.spacing_gain_miles, row.candidate_basis
    );
    StopCandidateRow {
        stop_id: row.candidate_id.clone(),
        name: row.candidate_name.clone(),
        state: "TBD".to_string(),
        lat: row.candidate_lat.clone(),
        lon: row.candidate_lon.clone(),
        requested_class: class,
        route_refs: denormalized_route_refs(&row.candidate_route_refs),
        stop_role,
        transfer_value: if row.intersection_route_count >= 2 {
            "medium".to_string()
        } else {
            "low".to_string()
        },
        freight_volume: "source_needed".to_string(),
        spacing_need: "high".to_string(),
        resilience_value: "source_needed".to_string(),
        energy_service: "planned".to_string(),
        land_ops_feasibility: "review_needed".to_string(),
        equity_community: "review_needed".to_string(),
        evidence_status,
        source_artifact: "data/beck-stop-sla-candidates.csv".to_string(),
        next_step,
    }
}

