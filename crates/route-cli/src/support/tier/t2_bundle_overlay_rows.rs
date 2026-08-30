//! Helper `t2_bundle_overlay_rows` (support::tier).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_overlay_rows(
    service_rows: &[T2ServiceSelectionRow],
    bundle_rows: &[NationalSegmentBundleRow],
    overlay_rows: &[GameT2ServiceOverlayRow],
) -> Vec<T2BundleOverlayRow> {
    let registry = route_network::BundleRegistry::new(
        bundle_rows
            .iter()
            .map(segment_bundle_from_national_row)
            .collect(),
    );
    let overlay_by_class = overlay_rows
        .iter()
        .map(|row| (row.service_class.clone(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let qualification_effects_by_bundle = bundle_rows
        .iter()
        .filter(|row| !row.qualification_effects.trim().is_empty())
        .map(|row| {
            (
                row.segment_bundle_id.clone(),
                row.qualification_effects.clone(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();

    service_rows
        .iter()
        .map(|service| {
            let route_bundles = registry.by_route_label(&service.route);
            let bundle = route_bundles.first().copied();
            let overlay = overlay_by_class.get(&service.beck_service_class);
            let service_class = if service.beck_service_class.trim().is_empty() {
                "unclassified".to_string()
            } else {
                service.beck_service_class.clone()
            };
            let (binding_status, next_artifact, validation_status) = match (bundle, overlay) {
                (Some(_), _) if service_class == "unclassified" => (
                    "service-class-held-known",
                    "data/game/t2-service-overlays.csv",
                    "review",
                ),
                (Some(bundle), Some(_))
                    if bundle
                        .validation_statuses
                        .iter()
                        .all(|status| status == "pass") =>
                {
                    ("bundle-bound", "data/game/t2-scenario-hooks.csv", "pass")
                }
                (Some(_), Some(_)) => (
                    "bundle-bound-review",
                    "data/national-segment-bundles.csv",
                    "review",
                ),
                (None, Some(_)) => (
                    "bundle-binding-pending",
                    "data/national-segment-bundles.csv",
                    "review",
                ),
                (_, None) => (
                    "service-class-overlay-pending",
                    "data/game/t2-service-overlays.csv",
                    "review",
                ),
            };
            T2BundleOverlayRow {
                tier: service.tier.clone(),
                region_id: service.region_id.clone(),
                route: service.route.clone(),
                segment_bundle_id: bundle
                    .map(|bundle| bundle.segment_bundle_id.clone())
                    .unwrap_or_default(),
                bundle_status: bundle
                    .map(|bundle| bundle.bundle_status.as_str().to_string())
                    .unwrap_or_else(|| "missing-bundle".to_string()),
                service_class,
                map_id: overlay.map(|row| row.map_id.clone()).unwrap_or_default(),
                scenario_hook: overlay
                    .map(|row| row.scenario_hook.clone())
                    .unwrap_or_default(),
                incident_lever: overlay
                    .map(|row| row.incident_lever.clone())
                    .unwrap_or_default(),
                upgrade_lever: overlay
                    .map(|row| row.upgrade_lever.clone())
                    .unwrap_or_default(),
                restitch_lever: overlay
                    .map(|row| row.restitch_lever.clone())
                    .unwrap_or_default(),
                release_gate: overlay
                    .map(|row| row.release_gate.clone())
                    .unwrap_or_default(),
                qualification_map_treatment: service.qualification_map_treatment.clone(),
                qualification_gate_policy: service.qualification_gate_policy.clone(),
                qualification_game_use: service.qualification_game_use.clone(),
                qualification_effects: merge_qualification_effects(
                    &service.qualification_effects,
                    bundle
                        .and_then(|bundle| {
                            qualification_effects_by_bundle.get(&bundle.segment_bundle_id)
                        })
                        .map(String::as_str)
                        .unwrap_or_default(),
                ),
                pavement_debt_cost_m: service.pavement_debt_cost_m,
                pavement_debt_class: service.pavement_debt_class.clone(),
                pavement_debt_basis: service.pavement_debt_basis.clone(),
                source_artifacts:
                    "data/t2-service-selection.csv;data/national-segment-bundles.csv;data/game/t2-service-overlays.csv"
                        .to_string(),
                binding_status: binding_status.to_string(),
                next_artifact: next_artifact.to_string(),
                validation_status: validation_status.to_string(),
            }
        })
        .collect()
}
