//! Helper `t2_service_overlay_diagnostic_decision_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_overlay_diagnostic_decision_rows(
    docket_rows: &[T2ServiceClassRepairDocketRow],
    target_rows: &[T2BundleOverlayRepairTargetRow],
    diagnostic_rows: &[T2ServiceDiagnosticQueueRow],
) -> Vec<T2ServiceOverlayDiagnosticDecisionRow> {
    let targets = target_rows
        .iter()
        .map(|row| (row.target_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let diagnostics_by_route = diagnostic_rows
        .iter()
        .filter(|row| !row.route.starts_with("__"))
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = docket_rows
        .iter()
        .filter(|row| row.service_repair_class == "service-overlay")
        .map(|row| {
            let target = targets.get(row.target_id.as_str());
            let diagnostic = diagnostics_by_route.get(&canonical_route_key(&row.route));
            let diagnostic_status = diagnostic
                .map(|row| row.diagnostic_status.clone())
                .unwrap_or_else(|| "missing-beck-t2-diagnostic".to_string());
            let diagnostic_action = diagnostic
                .map(|row| row.service_diagnostic_action.clone())
                .unwrap_or_else(|| "beck-diagnostic-missing".to_string());
            let blocks_claims = target
                .map(|row| row.blocks_claims.clone())
                .unwrap_or_else(|| "game;incident;publication;upgrade".to_string());
            let (overlay_decision, decision_reason, required_artifact, next_artifact) =
                if row.service_class == "unclassified" {
                    (
                    "held",
                    "service class remains unclassified until Beck T2 diagnostic chooses a class",
                    "data/beck-t2-diagnostics.csv",
                    "data/t2-service-class-repair-docket.csv",
                )
                } else {
                    (
                        "repair-needed",
                        "service class exists but overlay binding still requires replay",
                        "data/game/t2-service-overlays.csv",
                        "data/t2-game-ops-binding-decisions.csv",
                    )
                };
            T2ServiceOverlayDiagnosticDecisionRow {
                decision_id: format!(
                    "T2SERVICEOVERLAYDIAG-{}",
                    stable_id_fragment(&row.docket_id)
                ),
                docket_id: row.docket_id.clone(),
                target_id: row.target_id.clone(),
                route: row.route.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                bundle_status: row.bundle_status.clone(),
                current_service_class: row.service_class.clone(),
                diagnostic_status,
                diagnostic_action,
                overlay_decision: overlay_decision.to_string(),
                decision_reason: decision_reason.to_string(),
                qualification_effects: row.qualification_effects.clone(),
                blocks_claims,
                required_artifact: required_artifact.to_string(),
                next_artifact: next_artifact.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.segment_bundle_id.cmp(&right.segment_bundle_id))
    });
    rows
}

