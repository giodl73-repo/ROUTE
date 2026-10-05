//! Helper `segment_bundle_from_national_row`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn segment_bundle_from_national_row(
    row: &NationalSegmentBundleRow,
) -> route_network::SegmentBundle {
    route_network::SegmentBundle {
        segment_bundle_id: row.segment_bundle_id.clone(),
        bundle_role: row.bundle_role.clone(),
        member_segment_ids: semicolon_values(&row.member_segment_ids),
        stitch_group_ids: semicolon_values(&row.stitch_group_ids),
        current_tiers: semicolon_values(&row.current_tiers),
        current_zone_ids: semicolon_values(&row.current_zone_ids),
        route_labels: semicolon_values(&row.route_labels),
        state_scope: semicolon_values(&row.state_scope),
        evidence_state_scope: semicolon_values(&row.evidence_state_scope),
        geometry_state_scope: semicolon_values(&row.geometry_state_scope),
        bundle_aliases: semicolon_values(&row.bundle_aliases),
        source_artifacts: semicolon_values(&row.source_artifacts),
        registry_actions: Vec::new(),
        validation_statuses: vec![row.validation_status.clone()],
        bundle_status: route_network::BundleStatus::from_label(&row.bundle_status),
    }
}
