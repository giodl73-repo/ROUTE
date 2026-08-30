//! Helper `t2_beck_label_density_policy_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_label_density_policy_rows(
    review_rows: &[T2BeckLabelDensityReviewRow],
) -> Vec<T2BeckLabelDensityPolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "label-density-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLabelDensityPolicyRow {
            policy_id: format!("T2LABELPOLICY-{}", stable_id_fragment(&row.route)),
            label_review_id: row.label_review_id.clone(),
            route: row.route.clone(),
            trunk_pair: format!("{}-{}", row.start_trunk, row.end_trunk),
            service_class: row.service_class.clone(),
            label_density_per_100px: row.label_density_per_100px,
            density_band: t2_label_density_band(row.label_density_per_100px).to_string(),
            policy_basis: row.density_basis.clone(),
            label_policy_decision: "label-density-policy-authored-review".to_string(),
            render_treatment:
                "compress labels to trunk interfaces and preserve intermediate stops as unlabeled service beads"
                    .to_string(),
            promotion_treatment:
                "hold map promotion until accepted label-density simplification is replayed"
                    .to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-label-density-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
