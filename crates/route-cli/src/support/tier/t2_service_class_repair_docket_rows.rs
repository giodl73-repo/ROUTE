//! Helper `t2_service_class_repair_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_class_repair_docket_rows(
    target_rows: &[T2BundleOverlayRepairTargetRow],
    diagnostic_rows: &[T2ServiceDiagnosticQueueRow],
) -> Vec<T2ServiceClassRepairDocketRow> {
    let diagnostics_by_route = diagnostic_rows
        .iter()
        .filter(|row| !row.route.starts_with("__"))
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = target_rows
        .iter()
        .filter(|row| row.service_class == "unclassified" || row.repair_class == "service-class")
        .map(|row| {
            let diagnostic = diagnostics_by_route.get(&canonical_route_key(&row.route));
            let diagnostic_status = diagnostic
                .map(|row| row.diagnostic_status.as_str())
                .unwrap_or_default();
            let (service_repair_class, service_action, required_artifact, next_artifact, effect) =
                match diagnostic_status {
                    "local-relief-map-review" => (
                        "local-zone",
                        "hold-local-relief-below-national-game-overlay",
                        "data/t3-t4-pressure-intake.csv",
                        "data/t3-zone-render-board.csv",
                        "keeps local relief treatment out of national T2 game overlay until zone role is explicit",
                    ),
                    "missing-beck-t2-diagnostic" => (
                        "beck-diagnostic",
                        "author-beck-t2-diagnostic-before-service-class",
                        "data/beck-t2-diagnostics.csv",
                        "data/game/t2-service-overlays.csv",
                        "keeps service-class-held row blocked until Beck diagnostic chooses a class",
                    ),
                    _ => (
                        "service-overlay",
                        "repair-service-overlay-before-game-ops-binding",
                        "data/game/t2-service-overlays.csv",
                        "data/t2-game-ops-binding-decisions.csv",
                        "keeps service-class-held row blocked until overlay metadata is usable",
                    ),
                };
            T2ServiceClassRepairDocketRow {
                docket_id: format!("T2SERVICECLASSREPAIR-{}", stable_id_fragment(&row.target_id)),
                target_id: row.target_id.clone(),
                route: row.route.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                bundle_status: row.bundle_status.clone(),
                service_class: row.service_class.clone(),
                service_repair_class: service_repair_class.to_string(),
                service_action: service_action.to_string(),
                qualification_effects: row.qualification_effects.clone(),
                required_artifact: required_artifact.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: service_repair_optimizer_effect(effect, row),
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
