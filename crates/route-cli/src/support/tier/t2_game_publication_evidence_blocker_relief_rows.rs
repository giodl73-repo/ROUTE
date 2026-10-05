//! Helper `t2_game_publication_evidence_blocker_relief_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_evidence_blocker_relief_rows(
    acceptance_rows: &[T2GamePublicationEvidencePolicyAcceptanceRow],
) -> Vec<T2GamePublicationEvidenceBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "publication-evidence-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GamePublicationEvidenceBlockerReliefRow {
            relief_id: format!("T2GAMERELIEF-{}", stable_id_fragment(&row.scenario_id)),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            scenario_id: row.scenario_id.clone(),
            service_class: row.service_class.clone(),
            accepted_required_evidence: row.accepted_required_evidence.clone(),
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
    rows.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
    rows
}
