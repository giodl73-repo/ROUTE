//! Helper `t1_feedback_service_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_feedback_service_decision(
    row: &T2ServiceSelectionRow,
    has_t1_sla_pair: bool,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    if has_t1_sla_pair {
        return (
            "t1-sla-candidate",
            "evaluate-t1-sla-route-substitution",
            "named T1 SLA pair plus proof the lower-tier service improves that promise",
            "data/t1-line-selector.csv",
            "may reopen T1 line selection only as an SLA/stop/topology repair",
            "review",
        );
    }
    if row.selection_action == "closure-review-needs-beck-diagnostic" {
        return (
            "beck-diagnostic-needed",
            "add-beck-diagnostic-before-t1-feedback",
            "Beck T2 diagnostic plus named T1 SLA dependency before any T1 review",
            "data/beck-t2-diagnostics.csv",
            "holds below T1 until service geometry and promise dependency are proven",
            "review",
        );
    }
    (
        "no-t1-action",
        "keep-below-t1",
        "named T1 SLA pair, T1 stop obligation, or T1 topology repair witness",
        "data/t2-service-selection.csv",
        "score or regional service value alone cannot promote a route to T1",
        "pass",
    )
}

