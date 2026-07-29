//! Helper `tier_contact_witness_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_contact_witness_rows(
    rows: &[TierRegionRepairInputRow],
    diagnostics: &[route_map::BeckT2DiagnosticRow],
) -> Vec<TierContactWitnessRow> {
    let diagnostics_by_route = diagnostics
        .iter()
        .map(|row| (canonical_route_key(row.corridor), row))
        .collect::<std::collections::HashMap<_, _>>();

    rows.iter()
        .map(|row| {
            let (witness_type, evidence_status, validation_status) =
                tier_contact_witness_status(&row.repair_action);
            let beck_contact = diagnostics_by_route
                .get(&canonical_route_key(&row.route))
                .and_then(|diagnostic| t2_beck_contact_witness(row, diagnostic));
            TierContactWitnessRow {
                tier: row.tier.clone(),
                route: row.route.clone(),
                witness_type: beck_contact
                    .as_ref()
                    .map(|contact| contact.witness_type.clone())
                    .unwrap_or_else(|| witness_type.to_string()),
                node_class: beck_contact
                    .as_ref()
                    .map(|contact| contact.node_class.clone())
                    .unwrap_or_else(|| row.node_class.clone()),
                route_miles: row.route_miles,
                observed_t1_node_count: beck_contact
                    .as_ref()
                    .map(|contact| contact.observed_t1_node_count)
                    .unwrap_or(row.t1_node_count),
                observed_parent_trunks: beck_contact
                    .as_ref()
                    .map(|contact| contact.observed_parent_trunks.clone())
                    .unwrap_or_else(|| row.parent_trunks.clone()),
                observed_dual_contacts: beck_contact
                    .as_ref()
                    .map(|contact| contact.observed_dual_contacts)
                    .unwrap_or(row.contact_route_count),
                component_id: row.component_id,
                component_route_count: row.component_route_count,
                component_status: row.component_status.clone(),
                repair_action: beck_contact
                    .as_ref()
                    .map(|contact| contact.repair_action.clone())
                    .unwrap_or_else(|| row.repair_action.clone()),
                repair_basis: beck_contact
                    .as_ref()
                    .map(|contact| contact.repair_basis.clone())
                    .unwrap_or_else(|| row.repair_basis.clone()),
                evidence_status: beck_contact
                    .as_ref()
                    .map(|contact| contact.evidence_status.clone())
                    .unwrap_or_else(|| evidence_status.to_string()),
                required_artifact: beck_contact
                    .as_ref()
                    .map(|contact| contact.required_artifact.clone())
                    .unwrap_or_else(|| row.next_artifact.clone()),
                validation_status: beck_contact
                    .as_ref()
                    .map(|contact| contact.validation_status.clone())
                    .unwrap_or_else(|| validation_status.to_string()),
            }
        })
        .collect()
}

