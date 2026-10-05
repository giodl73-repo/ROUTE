//! Helper `beck_t2_diagnostics_gate_failure`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn beck_t2_diagnostics_gate_failure(review_flag: &str) -> bool {
    matches!(
        review_flag,
        "unstopped-t1-contact-review"
            | "parallel-spacing-review"
            | "split-anchor-review"
            | "dense-label-review"
    )
}
