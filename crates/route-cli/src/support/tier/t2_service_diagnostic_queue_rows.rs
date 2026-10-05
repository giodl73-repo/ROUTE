//! Helper `t2_service_diagnostic_queue_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_diagnostic_queue_rows(
    service_rows: &[T2ServiceSelectionRow],
    bundle_rows: &[NationalSegmentBundleRow],
) -> Vec<T2ServiceDiagnosticQueueRow> {
    let mut rows = service_rows
        .iter()
        .filter(|row| {
            matches!(
                row.selection_action.as_str(),
                "source-needed" | "closure-review-needs-beck-diagnostic"
            ) || row.selection_basis == "missing-beck-t2-diagnostic"
                || row.selection_basis == "closure-accepted-missing-beck-t2-diagnostic"
        })
        .flat_map(|row| {
            bundle_rows
                .iter()
                .filter(|bundle| {
                    bundle.bundle_status == "bundle-ready"
                        && national_bundle_matches_route(bundle, &row.route)
                })
                .map(|bundle| {
                    let (
                        diagnostic_status,
                        service_diagnostic_action,
                        next_artifact,
                        optimizer_effect,
                    ) = t2_service_diagnostic_contract(row, Some(bundle));
                    T2ServiceDiagnosticQueueRow {
                        route: row.route.clone(),
                        region_id: row.region_id.clone(),
                        segment_bundle_id: bundle.segment_bundle_id.clone(),
                        bundle_status: bundle.bundle_status.clone(),
                        selection_action: row.selection_action.clone(),
                        selection_basis: row.selection_basis.clone(),
                        qualification_effects: row.qualification_effects.clone(),
                        diagnostic_status: diagnostic_status.to_string(),
                        service_diagnostic_action: service_diagnostic_action.to_string(),
                        required_artifact: "data/t2-service-selection.csv".to_string(),
                        next_artifact: next_artifact.to_string(),
                        optimizer_effect: service_diagnostic_optimizer_effect(
                            optimizer_effect,
                            &row.qualification_effects,
                        ),
                        validation_status: "review".to_string(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    if rows.is_empty() {
        rows.push(T2ServiceDiagnosticQueueRow {
            route: "__all_t2_service_diagnostics__".to_string(),
            region_id: String::new(),
            segment_bundle_id: String::new(),
            bundle_status: "service-diagnostic-clear".to_string(),
            selection_action: "clear".to_string(),
            selection_basis: "no-missing-beck-t2-diagnostics".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            diagnostic_status: "service-diagnostic-clear".to_string(),
            service_diagnostic_action: "no-service-diagnostic-work-needed".to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "data/game/t2-bundle-overlays.csv".to_string(),
            optimizer_effect: "all T2 service rows have Beck diagnostic posture".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}
