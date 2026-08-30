//! Helper `tier_pavement_source_gap_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_source_gap_rows(
    graph: Option<&route_network::HighwayGraph>,
    docket_rows: &[TierPavementDocketRow],
) -> Vec<TierPavementSourceGapRow> {
    let mut builders = std::collections::BTreeMap::<String, TierPavementSourceGapBuilder>::new();

    for row in docket_rows {
        let builder = builders
            .entry(row.segment_bundle_id.clone())
            .or_insert_with(|| TierPavementSourceGapBuilder {
                tier: row.tier.clone(),
                route: row.route.clone(),
                region_id: row.region_id.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                stitch_group_id: row.stitch_group_id.clone(),
                ..Default::default()
            });
        builder.member_count += 1;
        if row.validation_status == "review" || row.pavement_status != "pavement-floor-pass" {
            builder.blocker_count += 1;
            builder.blocker_statuses.insert(row.pavement_status.clone());
            if !row.state.trim().is_empty() {
                builder.affected_states.insert(row.state.clone());
            }
            builder.affected_edge_ids.insert(row.edge_id);
            if !row.source_contract.trim().is_empty() {
                builder.source_contracts.insert(row.source_contract.clone());
            }
        }
    }

    builders
        .into_values()
        .filter(|builder| builder.blocker_count > 0)
        .map(|builder| {
            let blocker_statuses = join_string_set(&builder.blocker_statuses);
            let affected_states = if builder.affected_states.is_empty() {
                tier_pavement_route_state_scope(graph, &builder.route)
            } else {
                join_string_set(&builder.affected_states)
            };
            let affected_edge_ids = builder
                .affected_edge_ids
                .iter()
                .map(|edge_id| edge_id.to_string())
                .collect::<Vec<_>>()
                .join(";");
            let source_contract = join_string_set(&builder.source_contracts);
            let (source_action, next_artifact, optimizer_effect, validation_status) =
                tier_pavement_source_gap_decision(&blocker_statuses);
            TierPavementSourceGapRow {
                tier: builder.tier,
                route: builder.route,
                region_id: builder.region_id,
                segment_bundle_id: builder.segment_bundle_id,
                stitch_group_id: builder.stitch_group_id,
                member_count: builder.member_count,
                blocker_count: builder.blocker_count,
                blocker_statuses,
                affected_states,
                affected_edge_ids,
                source_contract,
                source_action: source_action.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: validation_status.to_string(),
            }
        })
        .collect()
}
