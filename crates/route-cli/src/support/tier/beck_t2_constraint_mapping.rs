//! Helper `beck_t2_constraint_mapping`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn beck_t2_constraint_mapping(
    review_flag: &str,
) -> (&'static str, &'static str, &'static str) {
    match review_flag {
        "unstopped-t1-contact-review" => (
            "beck_unstopped_contact",
            "add-transfer-stop-or-realign-contact",
            "data/beck-t2-diagnostics.csv",
        ),
        "parallel-spacing-review" => (
            "beck_parallel_spacing",
            "separate-merge-or-demote-parallel-service",
            "data/t2-parallel-service-queue.csv",
        ),
        "split-anchor-review" => (
            "beck_split_anchor",
            "add-split-anchor-stop-or-use-single-parent-color",
            "data/beck-t2-diagnostics.csv",
        ),
        "duplicate-service-review" => (
            "beck_duplicate_service",
            "merge-demote-or-prove-distinct-parent-service",
            "data/t2-service-selection.csv",
        ),
        "dense-label-review" | "dense-transfer-review" => (
            "beck_label_density",
            "space-labels-stops-or-split-service",
            "data/beck-t2-diagnostics.csv",
        ),
        "transfer-complexity-review" => (
            "beck_transfer_complexity",
            "simplify-transfer-spine-or-add-zone-map",
            "data/beck-t2-diagnostics.csv",
        ),
        "long-connector-review" => (
            "beck_long_connector",
            "review-long-connector-treatment",
            "data/beck-t2-diagnostics.csv",
        ),
        _ => (
            "beck_schematic_review",
            "review-beck-diagnostic",
            "data/beck-t2-diagnostics.csv",
        ),
    }
}
