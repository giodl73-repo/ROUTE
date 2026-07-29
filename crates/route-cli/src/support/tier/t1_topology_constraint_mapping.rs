//! Helper `t1_topology_constraint_mapping`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_topology_constraint_mapping(
    row: &T1TopologyRepairRow,
) -> (u8, &'static str, &'static str, String, &'static str) {
    match row.repair_type.as_str() {
        "shared-backbone-policy" => (
            13,
            "schematic_geometry",
            "claim-blocker",
            "review".to_string(),
            "map|publication",
        ),
        "national-relay-justification" => (
            1,
            "promise_portfolio",
            "selection-hard",
            "review".to_string(),
            "sla|publication",
        ),
        "held-candidate" => (
            3,
            "route_budget",
            "review",
            row.validation_status.clone(),
            "",
        ),
        _ => (
            5,
            "topology_connectivity",
            "review",
            row.validation_status.clone(),
            "map|publication",
        ),
    }
}

