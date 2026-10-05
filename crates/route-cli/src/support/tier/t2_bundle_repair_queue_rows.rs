//! Helper `t2_bundle_repair_queue_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_repair_queue_rows(
    candidate_rows: &[TierCandidateColumnRow],
    blocker_rows: &[T2BlockerClosureRow],
) -> Vec<T2BundleRepairQueueRow> {
    let blockers_by_route = blocker_rows
        .iter()
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::HashMap<_, _>>();
    let mut rows = candidate_rows
        .iter()
        .filter(|row| row.tier.eq_ignore_ascii_case("T2"))
        .filter(|row| row.evidence_status == "closure-bundle-pending")
        .map(|row| {
            let blocker = blockers_by_route.get(&canonical_route_key(&row.route));
            let bundle_status = if row.bundle_status.trim().is_empty() {
                blocker
                    .map(|blocker| blocker.bundle_status.clone())
                    .unwrap_or_else(|| "bundle-unchecked".to_string())
            } else {
                row.bundle_status.clone()
            };
            let bundle_action = if row.bundle_action.trim().is_empty() {
                blocker
                    .map(|blocker| blocker.bundle_action.clone())
                    .unwrap_or_else(|| "join blocker closure to bundle registry".to_string())
            } else {
                row.bundle_action.clone()
            };
            let (repair_action, next_artifact) =
                t2_bundle_repair_queue_action(bundle_status.as_str());
            let qualification_effects = merge_qualification_effects(
                &row.qualification_effects,
                blocker
                    .map(|blocker| blocker.qualification_effects.as_str())
                    .unwrap_or_default(),
            );
            T2BundleRepairQueueRow {
                route: row.route.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                bundle_status: bundle_status.clone(),
                bundle_action,
                contact_evidence_status: row.evidence_status.clone(),
                candidate_decision: row.column_decision.clone(),
                repair_class: blocker
                    .map(|blocker| blocker.blocker_class.clone())
                    .unwrap_or_else(|| "candidate-closure-bundle".to_string()),
                repair_action: repair_action.to_string(),
                required_artifact: row.required_artifact.clone(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: format!(
                    "{} remains out of T2 regionalizer until {}",
                    row.route, bundle_status
                ),
                qualification_effects,
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    if rows.is_empty() {
        rows.push(T2BundleRepairQueueRow {
            route: "__all_t2_bundle_repairs__".to_string(),
            segment_bundle_id: String::new(),
            bundle_status: "bundle-repair-clear".to_string(),
            bundle_action: "no pending closure-bundle rows".to_string(),
            contact_evidence_status: "no-closure-bundle-pending".to_string(),
            candidate_decision: "clear".to_string(),
            repair_class: "bundle-repair-clearance".to_string(),
            repair_action: "no-bundle-repair-needed".to_string(),
            required_artifact: "data/tier-candidate-columns.csv".to_string(),
            next_artifact: "data/t2-service-selection.csv".to_string(),
            optimizer_effect: "all bundle-ready candidate reviews may move to service diagnostics"
                .to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}
