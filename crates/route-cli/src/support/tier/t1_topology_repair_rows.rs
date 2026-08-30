//! Helper `t1_topology_repair_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_topology_repair_rows(rows: &[T1DesignReviewCsvRow]) -> Vec<T1TopologyRepairRow> {
    rows.iter()
        .filter(|row| !row.design_status.eq_ignore_ascii_case("accepted"))
        .map(|row| {
            let (repair_type, repair_basis, next_artifact, validation_status) =
                t1_topology_repair_contract(row);
            T1TopologyRepairRow {
                route: row.route.clone(),
                selected: row.selected,
                design_role: row.design_role.clone(),
                design_status: row.design_status.clone(),
                beck_review_flag: row.beck_review_flag.clone(),
                overlap_corridors: row.overlap_corridors.clone(),
                repair_type: repair_type.to_string(),
                repair_basis: repair_basis.to_string(),
                next_artifact: next_artifact.to_string(),
                next_action: row.next_design_action.clone(),
                validation_status: validation_status.to_string(),
            }
        })
        .collect()
}
