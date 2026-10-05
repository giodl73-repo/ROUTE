//! Helper `t2_beck_contact_witness`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_contact_witness(
    row: &TierRegionRepairInputRow,
    diagnostic: &route_map::BeckT2DiagnosticRow,
) -> Option<T2BeckContactWitness> {
    if !row.tier.eq_ignore_ascii_case("T2")
        || row.repair_action != "fix-graph-contact-or-demote"
        || diagnostic.service_action != "keep"
        || beck_t2_diagnostics_gate_failure(diagnostic.review_flag)
        || diagnostic.unstopped_t1_contact_count > 0
        || diagnostic.duplicate_service_count > 0
        || diagnostic.close_parallel_count > 0
    {
        return None;
    }

    let mut trunks = semicolon_values(&format!(
        "{};{}",
        diagnostic.start_trunk, diagnostic.end_trunk
    ))
    .into_iter()
    .map(|trunk| canonical_route_key(&trunk))
    .filter(|trunk| !trunk.is_empty())
    .collect::<Vec<_>>();
    trunks.sort();
    trunks.dedup();
    if trunks.len() < 2 {
        return None;
    }

    Some(T2BeckContactWitness {
        witness_type: "regionalizer-ready".to_string(),
        node_class: "trunk_connector".to_string(),
        observed_t1_node_count: trunks.len(),
        observed_parent_trunks: trunks.join(";"),
        observed_dual_contacts: trunks.len(),
        repair_action: "keep-for-regionalizer".to_string(),
        repair_basis: "beck-diagnostic-t1-contact".to_string(),
        evidence_status: "beck-contact-observed".to_string(),
        required_artifact: "data/tier-candidate-columns.csv".to_string(),
        validation_status: "pass".to_string(),
    })
}
