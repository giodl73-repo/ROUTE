//! Helper `beck_t2_constraint_penalty`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn beck_t2_constraint_penalty(row: &route_map::BeckT2DiagnosticRow) -> f64 {
    match row.review_flag {
        "unstopped-t1-contact-review" => row.unstopped_t1_contact_count.max(1) as f64,
        "parallel-spacing-review" => row.close_parallel_count.max(1) as f64,
        "duplicate-service-review" => row.duplicate_service_count.max(1) as f64,
        "dense-label-review" | "dense-transfer-review" => {
            (row.label_density_per_100px - 0.95).max(0.25)
        }
        "transfer-complexity-review" => 1.0 + row.transfer_stop_count.saturating_sub(4) as f64,
        "long-connector-review" => 1.0,
        "split-anchor-review" => 1.0,
        _ => 1.0,
    }
}
