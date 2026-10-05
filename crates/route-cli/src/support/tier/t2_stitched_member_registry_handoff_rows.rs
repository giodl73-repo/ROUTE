//! Helper `t2_stitched_member_registry_handoff_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_stitched_member_registry_handoff_rows(
    audit_rows: &[T2NationalBundleReadinessAuditRow],
    registry_rows: &[NationalSegmentRegistryRow],
    candidate_rows: &[TierSegmentCandidateRow],
) -> Vec<T2StitchedMemberRegistryHandoffRow> {
    let registry_by_bundle = registry_rows.iter().fold(
        std::collections::BTreeMap::<&str, Vec<&NationalSegmentRegistryRow>>::new(),
        |mut acc, row| {
            acc.entry(row.segment_bundle_id.as_str())
                .or_default()
                .push(row);
            acc
        },
    );
    let mut rows = audit_rows
        .iter()
        .filter(|row| {
            row.readiness_class == "stitched-member"
                && row.next_artifact == "data/national-segment-registry.csv"
        })
        .map(|audit| {
            let registry_members = registry_by_bundle
                .get(audit.segment_bundle_id.as_str())
                .map(|rows| rows.as_slice())
                .unwrap_or(&[]);
            let stitch_group_id = registry_members
                .first()
                .map(|row| row.stitch_group_id.clone())
                .unwrap_or_else(|| "missing-stitch-group".to_string());
            let candidate_bundle_member_count = candidate_rows
                .iter()
                .filter(|row| {
                    row.segment_bundle_id == audit.segment_bundle_id
                        && row.member_role == "stitched-member"
                })
                .count();
            let route_key = canonical_route_key(&audit.route);
            let candidate_route_member_count = candidate_rows
                .iter()
                .filter(|row| {
                    canonical_route_key(&row.route) == route_key
                        && row.member_role == "stitched-member"
                })
                .count();
            T2StitchedMemberRegistryHandoffRow {
                handoff_id: format!(
                    "T2STITCHEDREGISTRYHANDOFF-{}",
                    stable_id_fragment(&audit.audit_id)
                ),
                audit_id: audit.audit_id.clone(),
                route: audit.route.clone(),
                segment_bundle_id: audit.segment_bundle_id.clone(),
                stitch_group_id,
                current_registry_member_count: registry_members.len(),
                candidate_bundle_member_count,
                candidate_route_member_count,
                required_member_min: 2,
                handoff_decision: "held-for-member-expansion".to_string(),
                handoff_action: "expand-stitch-group-before-bundle-replay".to_string(),
                qualification_effects: audit.qualification_effects.clone(),
                blocked_claims_before: audit.blocked_claims_after.clone(),
                blocked_claims_after: audit.blocked_claims_after.clone(),
                blocker_delta: 0,
                next_artifact: "data/tier-segment-candidates.csv".to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
