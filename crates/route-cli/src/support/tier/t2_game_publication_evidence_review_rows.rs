//! Helper `t2_game_publication_evidence_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_evidence_review_rows(
    claim_rows: &[OptimizerClaimReviewRow],
    hook_rows: &[T2ScenarioHookRow],
) -> Vec<T2GamePublicationEvidenceReviewRow> {
    let Some(claim_row) = claim_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T2"
            && row.blocker_family == "game_ops_publication_readiness"
            && row.total_claim_blockers > 0
    }) else {
        return Vec::new();
    };
    let expected_scenarios = claim_row
        .representative_subjects
        .split(';')
        .filter(|scenario| !scenario.trim().is_empty())
        .map(str::to_string)
        .collect::<std::collections::BTreeSet<_>>();
    let mut rows = hook_rows
        .iter()
        .filter(|row| expected_scenarios.contains(row.scenario_id.as_str()))
        .map(|row| T2GamePublicationEvidenceReviewRow {
            game_review_id: format!("T2GAMEPUB-{}", stable_id_fragment(&row.scenario_id)),
            claim_review_id: claim_row.claim_review_id.clone(),
            scenario_id: row.scenario_id.clone(),
            service_class: row.service_class.clone(),
            t2_map_id: row.t2_map_id.clone(),
            player_decision: row.player_decision.clone(),
            evidence_hold: row.evidence_hold.clone(),
            review_decision: "publication-evidence-policy-required".to_string(),
            blocker_claims_before: claim_row.blocked_claims.clone(),
            blocker_claims_after: claim_row.blocked_claims.clone(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            required_evidence: t2_game_publication_required_evidence(&row.evidence_hold)
                .to_string(),
            next_artifact: "data/t2-game-publication-evidence-policy.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
    rows
}
