//! Helper `t1_feedback_bubble_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_feedback_bubble_decision(
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
            "evaluate-t1-sla-route-substitution-after-contact",
            "T2 contact witness plus named T1 SLA improvement",
            "data/t1-line-selector.csv",
            "may reopen T1 only after lower-tier contact and SLA dependency are both proven",
            "review",
        );
    }
    (
        "t2-contact-first",
        "require-t2-contact-before-any-t1-review",
        "T2 contact witness plus source-backed regional service value; T1 also requires named SLA dependency",
        "data/tier-contact-witnesses.csv",
        "holds pressure at T2 because no T1 promise depends on this route",
        "review",
    )
}

