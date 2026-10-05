//! Helper `t2_bubble_up_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bubble_up_review_rows(
    intake_rows: &[T3T4PressureIntakeRow],
) -> Vec<T2BubbleUpReviewRow> {
    intake_rows
        .iter()
        .filter(|row| row.intake_class == "bubble-up-t2-review")
        .map(|row| T2BubbleUpReviewRow {
            route: row.route.clone(),
            source_intake_class: row.intake_class.clone(),
            current_score: row.current_score,
            review_action: "require-t2-contact-witness-before-upgrade".to_string(),
            required_evidence: "T2 contact witness plus source-backed regional service value"
                .to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "may reopen T2 candidate review only after contact validation"
                .to_string(),
            validation_status: "review".to_string(),
        })
        .collect()
}
