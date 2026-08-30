//! Helper `t2_game_ops_binding_decision_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_binding_decision_rows(
    intake_rows: &[T2GameOpsBindingIntakeRow],
    overlay_rows: &[T2BundleOverlayRow],
) -> Vec<T2GameOpsBindingDecisionRow> {
    let overlays_by_subject = overlay_rows
        .iter()
        .map(|row| {
            let key = if row.segment_bundle_id.trim().is_empty() {
                row.route.clone()
            } else {
                row.segment_bundle_id.clone()
            };
            (key, row)
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = intake_rows
        .iter()
        .map(|row| {
            let overlay = overlays_by_subject
                .get(&row.subject_id)
                .or_else(|| overlays_by_subject.get(&row.route));
            let binding_status = overlay
                .map(|overlay| overlay.binding_status.clone())
                .unwrap_or_else(|| "bundle-binding-pending".to_string());
            let service_class = overlay
                .map(|overlay| overlay.service_class.clone())
                .unwrap_or_else(|| "unclassified".to_string());
            let bundle_status = overlay
                .map(|overlay| overlay.bundle_status.clone())
                .unwrap_or_else(|| "missing-bundle".to_string());
            let overlay_validation_status = overlay
                .map(|overlay| overlay.validation_status.as_str())
                .unwrap_or("review");
            let qualification_gate_policy = overlay
                .map(|overlay| overlay.qualification_gate_policy.clone())
                .unwrap_or_default();
            let qualification_game_use = overlay
                .map(|overlay| overlay.qualification_game_use.clone())
                .unwrap_or_default();
            let (decision, reason, next_artifact, validation_status) =
                if binding_status == "bundle-bound" && overlay_validation_status == "pass" {
                    (
                        "bound",
                        "bundle overlay is bound and passed validation",
                        "data/game/t2-scenario-hooks.csv",
                        "pass",
                    )
                } else {
                    match binding_status.as_str() {
                        "bundle-bound" | "bundle-bound-review" => (
                            "repair-needed",
                            "bundle id exists but bundle validation remains under review",
                            "data/national-segment-bundles.csv",
                            "review",
                        ),
                        "service-class-overlay-pending" | "service-class-held-known" => (
                            "held",
                            "service class overlay is missing or held",
                            "data/game/t2-service-overlays.csv",
                            "review",
                        ),
                        "bundle-binding-pending" => (
                            "repair-needed",
                            "route is not bound to a usable segment bundle",
                            "data/national-segment-bundles.csv",
                            "review",
                        ),
                        _ => (
                            "held",
                            "binding status requires manual review",
                            "data/game/t2-bundle-overlays.csv",
                            "review",
                        ),
                    }
                };
            T2GameOpsBindingDecisionRow {
                decision_id: format!("T2GAMEOPSDECISION-{}", stable_id_fragment(&row.intake_id)),
                intake_id: row.intake_id.clone(),
                subject_id: row.subject_id.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                route: row.route.clone(),
                service_class,
                bundle_status,
                binding_status,
                qualification_effects: merge_qualification_effects(
                    &row.qualification_effects,
                    overlay
                        .map(|overlay| overlay.qualification_effects.as_str())
                        .unwrap_or_default(),
                ),
                qualification_gate_policy,
                qualification_game_use,
                decision: decision.to_string(),
                decision_reason: reason.to_string(),
                blocks_claims: if validation_status == "pass" {
                    String::new()
                } else {
                    row.blocked_claims.clone()
                },
                next_artifact: next_artifact.to_string(),
                validation_status: validation_status.to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.subject_id.cmp(&right.subject_id))
    });
    rows
}
