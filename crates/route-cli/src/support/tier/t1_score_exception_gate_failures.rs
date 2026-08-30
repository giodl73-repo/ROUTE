//! Helper `t1_score_exception_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_score_exception_gate_failures(
    review_rows: &[T1DesignReviewCsvRow],
    exception_rows: &[T1ScoreExceptionRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let exceptions = exception_rows
        .iter()
        .map(|row| (normalise_designation(&row.route), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let valid_decisions = ["keep", "conditional_keep", "demote", "replace"];

    for row in exception_rows {
        let decision = row.decision.trim();
        if row.route.trim().is_empty()
            || decision.is_empty()
            || row.exception_type.trim().is_empty()
            || row.rationale.trim().is_empty()
            || row.evidence_status.trim().is_empty()
            || row.artifact.trim().is_empty()
            || row.next_selector_action.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete score-exception contract",
                row.route
            ));
        }
        if !valid_decisions.contains(&decision) {
            failures.push(format!(
                "{} has invalid decision {}",
                row.route, row.decision
            ));
        }
        if matches!(decision, "demote" | "replace") && row.replacement_candidate.trim().is_empty() {
            failures.push(format!(
                "{} decision {} requires replacement_candidate",
                row.route, row.decision
            ));
        }
    }

    for row in review_rows
        .iter()
        .filter(|row| row.selected && row.design_role == "score-backbone-exception")
    {
        if !exceptions.contains_key(&normalise_designation(&row.route)) {
            failures.push(format!(
                "{} is a selected score-backbone exception without an exception decision",
                row.route
            ));
        }
    }
    for row in review_rows.iter().filter(|row| row.selected) {
        if let Some(exception) = exceptions.get(&normalise_designation(&row.route)) {
            if matches!(exception.decision.trim(), "demote" | "replace") {
                failures.push(format!(
                    "{} is selected but score exception decision is {}",
                    row.route, exception.decision
                ));
            }
        }
    }

    failures
}
