//! Helper `t1_topology_repair_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_topology_repair_contract(
    row: &T1DesignReviewCsvRow,
) -> (&'static str, &'static str, &'static str, &'static str) {
    if row.selected
        && row
            .beck_review_flag
            .trim()
            .eq_ignore_ascii_case("overlap-review")
    {
        (
            "shared-backbone-policy",
            "selected-t1-route-shares-beck-segment",
            "data/t1-design-policy-actions.csv",
            "review",
        )
    } else if row.selected && row.design_status.eq_ignore_ascii_case("policy-review") {
        (
            "national-relay-justification",
            "selected-score-exception-needs-national-role-proof",
            "data/t1-score-exceptions.csv",
            "review",
        )
    } else if !row.selected && row.design_status.eq_ignore_ascii_case("held") {
        (
            "held-candidate",
            "outside-current-t1-budget-or-demoted",
            "data/t1-line-selector.csv",
            "pass",
        )
    } else {
        (
            "unclassified-topology-review",
            "non-accepted-design-status",
            "data/t1-topology-repairs.csv",
            "review",
        )
    }
}

