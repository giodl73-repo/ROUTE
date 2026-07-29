//! Helper `t2_bubble_up_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bubble_up_review_gate_failures(rows: &[T2BubbleUpReviewRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 bubble-up review rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.review_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete T2 bubble-up review", row.route));
        }
    }
    failures
}

