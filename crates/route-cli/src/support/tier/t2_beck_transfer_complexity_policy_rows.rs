//! Helper `t2_beck_transfer_complexity_policy_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_transfer_complexity_policy_rows(
    review_rows: &[T2BeckTransferComplexityReviewRow],
) -> Vec<T2BeckTransferComplexityPolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "transfer-complexity-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| {
            let trunk_pair = format!("{}-{}", row.start_trunk, row.end_trunk);
            T2BeckTransferComplexityPolicyRow {
                policy_id: format!("T2TRANSFERPOLICY-{}", stable_id_fragment(&row.route)),
                transfer_review_id: row.transfer_review_id.clone(),
                route: row.route.clone(),
                trunk_pair,
                service_class: row.service_class.clone(),
                transfer_stop_count: row.transfer_stop_count,
                stop_count: row.stop_count,
                complexity_band: t2_transfer_complexity_band(row.transfer_stop_count).to_string(),
                policy_basis: row.complexity_basis.clone(),
                transfer_policy_decision: "transfer-simplification-policy-authored-review"
                    .to_string(),
                render_treatment:
                    "compress transfer emphasis to trunk interfaces and preserve local stops as unlabeled service beads"
                        .to_string(),
                promotion_treatment:
                    "hold map promotion until accepted transfer simplification is replayed"
                        .to_string(),
                publication_status: "held-pending-policy-acceptance".to_string(),
                blocker_claims_before: row.blocker_claims_after.clone(),
                blocker_claims_after: row.blocker_claims_after.clone(),
                blocker_count_before: row.blocker_count_after,
                blocker_count_after: row.blocker_count_after,
                claim_blocker_delta: 0,
                next_artifact: "data/t2-beck-transfer-complexity-policy-acceptance.csv"
                    .to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
