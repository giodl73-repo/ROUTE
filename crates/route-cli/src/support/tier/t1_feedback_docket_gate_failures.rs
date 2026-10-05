//! Helper `t1_feedback_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_feedback_docket_gate_failures(rows: &[T1FeedbackDocketRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T1 feedback docket rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.source_surface.trim().is_empty()
            || row.source_action.trim().is_empty()
            || row.t1_feedback_class.trim().is_empty()
            || row.t1_feedback_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete T1 feedback row", row.route));
        }
        if row.t1_feedback_class == "t1-sla-candidate"
            && (row.t1_sla_pair_count == 0 || row.t1_sla_pairs.trim().is_empty())
        {
            failures.push(format!(
                "{} promoted to T1 candidate without named SLA pair",
                row.route
            ));
        }
        if row.t1_feedback_class != "t1-sla-candidate"
            && row.t1_feedback_action.contains("t1-sla-route-substitution")
        {
            failures.push(format!(
                "{} has T1 substitution action outside t1-sla-candidate class",
                row.route
            ));
        }
    }
    failures
}
