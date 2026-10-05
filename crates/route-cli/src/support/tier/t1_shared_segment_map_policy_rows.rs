//! Helper `t1_shared_segment_map_policy_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_shared_segment_map_policy_rows(
    schematic_rows: &[T1SchematicGeometryClaimReviewRow],
) -> Vec<T1SharedSegmentMapPolicyRow> {
    let mut builders = std::collections::BTreeMap::<String, T1SharedSegmentPolicyBuilder>::new();
    for row in schematic_rows.iter().filter(|row| {
        row.policy_action == "resolve-shared-segment-map-policy"
            && row.review_decision == "shared-segment-map-policy-required"
            && row.claim_blocker_delta == 0
    }) {
        let pair = shared_segment_pair_id(&row.route, &row.overlap_corridors);
        let builder = builders.entry(pair).or_default();
        builder.routes.insert(row.route.clone());
        builder
            .routes
            .insert(route_display_key(&row.overlap_corridors));
        builder
            .source_review_ids
            .insert(row.schematic_review_id.clone());
        builder
            .blocker_claims
            .insert(row.blocker_claims_after.clone());
        builder.blocker_count += row.blocker_count_after;
        builder.policy_basis.insert(row.required_policy.clone());
        builder
            .design_treatments
            .insert(row.design_treatment.clone());
    }

    builders
        .into_iter()
        .map(|(route_pair, builder)| {
            let affected_routes = builder.routes.iter().cloned().collect::<Vec<_>>();
            let primary_route = affected_routes.first().cloned().unwrap_or_default();
            let overlap_route = affected_routes.get(1).cloned().unwrap_or_default();
            T1SharedSegmentMapPolicyRow {
                policy_id: format!("T1SHAREDSEG-{}", stable_id_fragment(&route_pair)),
                route_pair,
                primary_route,
                overlap_route,
                affected_routes: affected_routes.join(";"),
                source_review_ids: builder
                    .source_review_ids
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(";"),
                policy_basis: builder
                    .policy_basis
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(";"),
                map_policy_decision: "shared-segment-policy-authored-review".to_string(),
                render_treatment:
                    "represent as interlined trunk service or split at selected transfer stops"
                        .to_string(),
                selector_treatment: "keep both selected promise-spine routes pending acceptance"
                    .to_string(),
                publication_status: "held-pending-policy-acceptance".to_string(),
                blocker_claims_before: builder
                    .blocker_claims
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(";"),
                blocker_claims_after: builder
                    .blocker_claims
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(";"),
                blocker_count_before: builder.blocker_count,
                blocker_count_after: builder.blocker_count,
                claim_blocker_delta: 0,
                next_artifact: "data/t1-shared-segment-policy-acceptance.csv".to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect()
}
