//! Helper `tier_region_repair_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_region_repair_rows(rows: &[TierRegionWorkloadRow]) -> Vec<TierRegionRepairRow> {
    rows.iter()
        .map(|row| TierRegionRepairRow {
            tier: row.tier.clone(),
            route: row.route.clone(),
            node_class: row.node_class.clone(),
            route_miles: row.route_miles,
            t1_node_count: row.t1_node_count,
            parent_trunks: row.parent_trunks.clone(),
            contact_route_count: row.contact_route_count,
            component_id: row.component_id,
            component_route_count: row.component_route_count,
            component_status: row.component_status.clone(),
            repair_action: row.repair_action.clone(),
            repair_basis: row.repair_basis.clone(),
            next_artifact: tier_region_next_artifact(&row.repair_action).to_string(),
        })
        .collect()
}

