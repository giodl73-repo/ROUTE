//! Helper `t1_feedback_intake_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_feedback_intake_decision(
    row: &T3T4PressureIntakeRow,
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
            "lower-tier contact witness plus named T1 SLA improvement",
            "data/t1-line-selector.csv",
            "may reopen T1 only as a promise-preserving repair",
            "review",
        );
    }
    if row.current_score >= T1_THRESHOLD - 5.0 {
        return (
            "reject-t1-score-only-promotion",
            "reject-score-only-t1-promotion",
            "named T1 SLA pair, T1 stop obligation, or T1 topology repair witness",
            "data/t1-sla-pairs.csv",
            "near-threshold score is visible but cannot override the T1 promise portfolio",
            "pass",
        );
    }
    (
        "t2-contact-first",
        "require-t2-contact-before-any-t1-review",
        "T2 contact witness plus source-backed regional service value; T1 also requires named SLA dependency",
        "data/tier-contact-witnesses.csv",
        "holds pressure below T1 because no T1 promise depends on this route",
        "review",
    )
}

