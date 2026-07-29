//! Helper `t2_service_selection_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_selection_decision(
    row: &T2RegionalizerRow,
    diagnostic: Option<&route_map::BeckT2DiagnosticRow>,
) -> (String, String, String) {
    let Some(diagnostic) = diagnostic else {
        if row.evidence_status == "closure-accepted" && row.treatment_status == "review-treatment" {
            return (
                "closure-review-needs-beck-diagnostic".to_string(),
                "closure-accepted-missing-beck-t2-diagnostic".to_string(),
                "review".to_string(),
            );
        }
        return (
            "source-needed".to_string(),
            "missing-beck-t2-diagnostic".to_string(),
            "review".to_string(),
        );
    };

    if diagnostic.unstopped_t1_contact_count > 0 {
        return (
            "repair-stop-contact-before-selection".to_string(),
            "unstopped-t1-contact".to_string(),
            "review".to_string(),
        );
    }
    if diagnostic.duplicate_service_count > 0 {
        return (
            "split-or-demote-duplicate-service".to_string(),
            "duplicate-beck-service".to_string(),
            "review".to_string(),
        );
    }
    if diagnostic.close_parallel_count > 0 {
        return (
            "split-parallel-service".to_string(),
            "close-parallel-beck-service".to_string(),
            "review".to_string(),
        );
    }
    if row.treatment_status == "selected-treatment" && diagnostic.service_action == "keep" {
        return (
            "keep-service-column".to_string(),
            "diagnostic-backed-distinct-service".to_string(),
            "pass".to_string(),
        );
    }
    if row.treatment_status == "review-treatment" {
        return (
            "parent-region-review".to_string(),
            "regionalizer-review-treatment".to_string(),
            "review".to_string(),
        );
    }

    (
        "review-service-column".to_string(),
        "service-diagnostic-action-mismatch".to_string(),
        "review".to_string(),
    )
}

