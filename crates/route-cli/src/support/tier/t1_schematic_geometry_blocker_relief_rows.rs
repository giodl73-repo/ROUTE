//! Helper `t1_schematic_geometry_blocker_relief_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_schematic_geometry_blocker_relief_rows(
    acceptance_rows: &[T1SharedSegmentPolicyAcceptanceRow],
) -> Vec<T1SchematicGeometryBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_status == "accepted-policy-ready-for-relief-replay"
                && row.publication_status_after == "held-pending-blocker-relief-replay"
                && row.claim_blocker_delta == 0
        })
        .map(|row| T1SchematicGeometryBlockerReliefRow {
            relief_id: format!(
                "T1SCHEMATICRELIEF-{}",
                stable_id_fragment(&row.acceptance_id)
            ),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route_pair: row.route_pair.clone(),
            affected_routes: row.affected_routes.clone(),
            accepted_render_treatment: row.accepted_render_treatment.clone(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: String::new(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: 0,
            claim_blocker_delta: -(row.blocker_count_after as isize),
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route_pair.cmp(&right.route_pair));
    rows
}

