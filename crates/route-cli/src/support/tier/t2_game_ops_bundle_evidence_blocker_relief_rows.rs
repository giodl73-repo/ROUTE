//! Helper `t2_game_ops_bundle_evidence_blocker_relief_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_evidence_blocker_relief_rows(
    acceptance_rows: &[T2GameOpsBundleEvidencePolicyAcceptanceRow],
) -> Vec<T2GameOpsBundleEvidenceBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "bundle-evidence-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GameOpsBundleEvidenceBlockerReliefRow {
            relief_id: format!(
                "T2GAMEOPSBUNDLERELIEF-{}",
                stable_id_fragment(&row.acceptance_id)
            ),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            accepted_required_evidence: row.accepted_required_evidence.clone(),
            qualification_effects: row.qualification_effects.clone(),
            qualification_gate_policy: row.qualification_gate_policy.clone(),
            qualification_game_use: row.qualification_game_use.clone(),
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
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.segment_bundle_id.cmp(&right.segment_bundle_id))
    });
    rows
}

