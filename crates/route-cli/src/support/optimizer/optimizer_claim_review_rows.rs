//! Helper `optimizer_claim_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_claim_review_rows(
    backlog_rows: &[OptimizerResidualBlockerBacklogRow],
) -> Vec<OptimizerClaimReviewRow> {
    let mut rows = backlog_rows
        .iter()
        .filter(|row| {
            row.priority_class == "P1-claim-blocker"
                && row.next_wave == "optimizer-claim-review"
                && row.total_claim_blockers > 0
        })
        .map(|row| OptimizerClaimReviewRow {
            claim_review_id: format!("OCR-{}", stable_id_fragment(&row.backlog_id)),
            backlog_id: row.backlog_id.clone(),
            priority_class: row.priority_class.clone(),
            blocker_family: row.blocker_family.clone(),
            tier: row.tier.clone(),
            blocked_claims: row.blocked_claims.clone(),
            subject_count: row.subject_count,
            route_count: row.route_count,
            total_claim_blockers: row.total_claim_blockers,
            representative_routes: row.representative_routes.clone(),
            representative_subjects: row.representative_subjects.clone(),
            evidence_artifacts: row.next_artifacts.clone(),
            review_decision: "held-for-source-specific-claim-review".to_string(),
            blocker_claims_before: row.blocked_claims.clone(),
            blocker_claims_after: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_artifact: row.next_artifacts.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.tier
            .cmp(&right.tier)
            .then_with(|| left.blocker_family.cmp(&right.blocker_family))
    });
    rows
}
