//! Helper `national_segment_bundle_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn national_segment_bundle_rows(
    registry_rows: &[NationalSegmentRegistryRow],
) -> Vec<NationalSegmentBundleRow> {
    let mut qualification_effects_by_bundle =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    let members = registry_rows
        .iter()
        .map(|row| {
            insert_pipe_values(
                qualification_effects_by_bundle
                    .entry(row.segment_bundle_id.clone())
                    .or_default(),
                &row.qualification_effects,
            );
            route_network::SegmentBundleMember {
                national_segment_id: row.national_segment_id.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                bundle_role: row.bundle_role.clone(),
                stitch_group_id: row.stitch_group_id.clone(),
                current_tier: row.current_tier.clone(),
                current_zone_id: row.current_zone_id.clone(),
                route_label: row.route_label.clone(),
                state_scope: row.state_scope.clone(),
                evidence_state_scope: row.evidence_state_scope.clone(),
                geometry_state_scope: row.geometry_state_scope.clone(),
                bundle_aliases: row.bundle_aliases.clone(),
                source_artifacts: row.source_artifacts.clone(),
                registry_action: row.registry_action.clone(),
                validation_status: row.validation_status.clone(),
                member_segment_ids: row.member_segment_ids.clone(),
            }
        })
        .collect::<Vec<_>>();

    route_network::build_segment_bundles(&members)
        .into_iter()
        .map(|bundle| {
            let (bundle_action, next_artifact) =
                route_network::bundle_action(bundle.bundle_status, &bundle.registry_actions);
            let segment_bundle_id = bundle.segment_bundle_id.clone();
            NationalSegmentBundleRow {
                segment_bundle_id,
                bundle_role: bundle.bundle_role,
                member_count: bundle.member_segment_ids.len(),
                member_segment_ids: bundle.member_segment_ids.join(";"),
                stitch_group_ids: bundle.stitch_group_ids.join(";"),
                current_tiers: bundle.current_tiers.join(";"),
                current_zone_ids: bundle.current_zone_ids.join(";"),
                route_labels: bundle.route_labels.join(";"),
                state_scope: bundle.state_scope.join(";"),
                evidence_state_scope: bundle.evidence_state_scope.join(";"),
                geometry_state_scope: bundle.geometry_state_scope.join(";"),
                bundle_aliases: bundle.bundle_aliases.join(";"),
                source_artifacts: bundle.source_artifacts.join(";"),
                bundle_status: bundle.bundle_status.as_str().to_string(),
                bundle_action: bundle_action.to_string(),
                qualification_effects: qualification_effects_by_bundle
                    .get(&bundle.segment_bundle_id)
                    .map(join_pipe_set)
                    .unwrap_or_default(),
                next_artifact: next_artifact.to_string(),
                validation_status: bundle.bundle_status.validation_status().to_string(),
            }
        })
        .collect()
}
