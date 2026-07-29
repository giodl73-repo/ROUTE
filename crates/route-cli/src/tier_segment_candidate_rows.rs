//! Extracted helper `tier_segment_candidate_rows` from main.
use super::*;

pub(crate) fn tier_segment_candidate_rows(
    graph: &route_network::HighwayGraph,
    t1_rows: &[T1LineSelectorInputRow],
    t2_rows: &[T2ServiceSelectionRow],
    repair_rows: &[T2BundleRepairQueueRow],
    route_family_rows: &[T2RouteFamilySplitRow],
) -> Vec<TierSegmentCandidateRow> {
    let split_service_families = route_family_rows
        .iter()
        .filter(|row| row.family_action == "split-numbered-service-family")
        .map(|row| canonical_route_key(&row.route))
        .collect::<std::collections::BTreeSet<_>>();
    let route_family_qualification_effects = route_family_rows
        .iter()
        .filter(|row| !row.qualification_effects.trim().is_empty())
        .map(|row| {
            (
                canonical_route_key(&row.route),
                row.qualification_effects.clone(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut service_rows = Vec::<(&str, &str, String, String, String, String, String)>::new();
    for row in t1_rows.iter().filter(|row| row.selected) {
        service_rows.push((
            "T1",
            "t1-line-selector",
            "national".to_string(),
            row.route.clone(),
            row.selected_stops.clone(),
            "selected T1 SLA promise route; decompose into graph segment members before bundle stitching"
                .to_string(),
            String::new(),
        ));
    }
    for row in t2_rows.iter().filter(|row| {
        matches!(
            row.selection_action.as_str(),
            "keep-service-column" | "parent-region-review" | "source-needed"
        )
    }) {
        service_rows.push((
            "T2",
            "t2-service-selection",
            row.region_id.clone(),
            row.route.clone(),
            row.parent_trunks.clone(),
            format!(
                "{}; {}; {}",
                row.beck_service_class, row.qualification_basis, row.selection_basis
            ),
            merge_qualification_effects(
                &row.qualification_effects,
                route_family_qualification_effects
                    .get(&canonical_route_key(&row.route))
                    .map(String::as_str)
                    .unwrap_or_default(),
            ),
        ));
    }
    for row in repair_rows
        .iter()
        .filter(|row| row.next_artifact == "data/national-segment-bundles.csv")
    {
        service_rows.push((
            "T2",
            "t2-bundle-repair-queue",
            "bundle-repair".to_string(),
            row.route.clone(),
            row.repair_class.clone(),
            format!(
                "{}; {}; {}",
                row.bundle_status, row.bundle_action, row.repair_action
            ),
            row.qualification_effects.clone(),
        ));
    }

    let mut rows = Vec::new();
    for (
        tier,
        source_selector,
        region_id,
        route,
        selector_basis,
        action_basis,
        qualification_effects,
    ) in service_rows
    {
        let route_key = normalise_designation(&route);
        let split_by_state =
            tier == "T2" && split_service_families.contains(&canonical_route_key(&route_key));
        let mut edges = graph.route_edges(&route_key).to_vec();
        edges.sort_by_key(|edge_idx| graph.graph[*edge_idx].id);
        let edge_count = edges.len();
        for (edge_sequence, edge_idx) in edges.into_iter().enumerate() {
            let edge = &graph.graph[edge_idx];
            let edge_state = route_network::infer_edge_state(edge);
            let bundle_scope = if split_by_state {
                edge_state.clone()
            } else {
                String::new()
            };
            let segment_bundle_id =
                tier_candidate_bundle_id(tier, &region_id, &route_key, &bundle_scope);
            let stitch_group_id =
                tier_candidate_stitch_group_id(tier, &region_id, &route_key, &bundle_scope);
            rows.push(TierSegmentCandidateRow {
                tier: tier.to_string(),
                source_selector: source_selector.to_string(),
                region_id: region_id.clone(),
                route: route_key.clone(),
                edge_id: edge.id,
                edge_sequence: edge_sequence + 1,
                national_segment_id: tier_candidate_segment_id(edge),
                segment_bundle_id: segment_bundle_id.clone(),
                stitch_group_id: stitch_group_id.clone(),
                member_role: if edge_count > 1 {
                    "stitched-member".to_string()
                } else {
                    "single-member-candidate".to_string()
                },
                state: edge_state,
                length_miles: rounded_score(edge.length_miles),
                aadt: edge
                    .aadt
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                lane_count: edge
                    .lane_count
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                route_aliases: tier_candidate_aliases(tier, &region_id, &route_key, &bundle_scope),
                selector_basis: selector_basis.clone(),
                candidate_action: action_basis.clone(),
                qualification_effects: qualification_effects.clone(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            });
        }
    }
    rows.sort_by(|a, b| {
        a.tier
            .cmp(&b.tier)
            .then_with(|| a.region_id.cmp(&b.region_id))
            .then_with(|| a.route.cmp(&b.route))
            .then_with(|| a.edge_sequence.cmp(&b.edge_sequence))
            .then_with(|| a.edge_id.cmp(&b.edge_id))
    });
    rows
}

