//! Extracted helper `national_segment_registry_rows` from main.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn national_segment_registry_rows(
    board_rows: &[T3ZoneRenderBoardRow],
    placement_rows: &[T3ZoneStopPlacementRow],
    segment_rows: &[TierSegmentCandidateRow],
    pavement_rows: &[TierPavementDocketRow],
) -> Vec<NationalSegmentRegistryRow> {
    let mut builders = std::collections::BTreeMap::<String, NationalSegmentRegistryBuilder>::new();
    let pavement_by_member = pavement_rows
        .iter()
        .map(|row| {
            (
                national_segment_member_key(&row.segment_bundle_id, &row.national_segment_id),
                row,
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();

    for row in board_rows {
        let builder = builders
            .entry(national_segment_member_key(
                &row.segment_bundle_id,
                &row.national_segment_id,
            ))
            .or_insert_with(|| NationalSegmentRegistryBuilder {
                national_segment_id: row.national_segment_id.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                bundle_role: "single-segment".to_string(),
                stitch_group_id: row.stitch_group_id.clone(),
                zone_id: row.zone_id.clone(),
                current_tier: "T3".to_string(),
                route: row.route.clone(),
                ..Default::default()
            });
        merge_segment_identity(
            builder,
            &row.segment_bundle_id,
            &row.stitch_group_id,
            &row.zone_id,
            "T3",
            &row.route,
        );
        builder.board_layers.insert(row.board_layer.clone());
        insert_semicolon_values(&mut builder.source_artifacts, &row.source_artifact);
        insert_semicolon_values(&mut builder.segment_aliases, &row.segment_aliases);
        insert_semicolon_values(&mut builder.bundle_aliases, &row.segment_aliases);
        builder
            .validation_statuses
            .insert(row.validation_status.clone());
    }

    for row in placement_rows {
        let builder = builders
            .entry(national_segment_member_key(
                &row.segment_bundle_id,
                &row.national_segment_id,
            ))
            .or_insert_with(|| NationalSegmentRegistryBuilder {
                national_segment_id: row.national_segment_id.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                bundle_role: "single-segment".to_string(),
                stitch_group_id: row.stitch_group_id.clone(),
                zone_id: row.zone_id.clone(),
                current_tier: "T3".to_string(),
                route: row.route.clone(),
                ..Default::default()
            });
        merge_segment_identity(
            builder,
            &row.segment_bundle_id,
            &row.stitch_group_id,
            &row.zone_id,
            "T3",
            &row.route,
        );
        builder.board_layers.insert("stop-placement".to_string());
        insert_semicolon_values(&mut builder.source_artifacts, &row.source_artifact);
        insert_semicolon_values(&mut builder.segment_aliases, &row.segment_aliases);
        insert_semicolon_values(&mut builder.bundle_aliases, &row.segment_aliases);
        insert_semicolon_values(&mut builder.evidence_state_scope, &row.state_scope);
        builder
            .stop_placement_status
            .insert(row.placement_status.clone());
        builder
            .validation_statuses
            .insert(row.validation_status.clone());
    }

    for row in segment_rows {
        let builder = builders
            .entry(national_segment_member_key(
                &row.segment_bundle_id,
                &row.national_segment_id,
            ))
            .or_insert_with(|| NationalSegmentRegistryBuilder {
                national_segment_id: row.national_segment_id.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                bundle_role: tier_segment_bundle_role(row).to_string(),
                stitch_group_id: row.stitch_group_id.clone(),
                zone_id: row.region_id.clone(),
                current_tier: row.tier.clone(),
                route: row.route.clone(),
                ..Default::default()
            });
        merge_segment_identity(
            builder,
            &row.segment_bundle_id,
            &row.stitch_group_id,
            &row.region_id,
            &row.tier,
            &row.route,
        );
        insert_non_empty_string(&mut builder.evidence_state_scope, &row.state);
        insert_non_empty_string(&mut builder.geometry_state_scope, &row.state);
        builder
            .board_layers
            .insert("tier-segment-candidate".to_string());
        insert_semicolon_values(
            &mut builder.source_artifacts,
            "data/tier-segment-candidates.csv",
        );
        insert_semicolon_values(&mut builder.segment_aliases, &row.route_aliases);
        insert_semicolon_values(&mut builder.bundle_aliases, &row.route_aliases);
        insert_pipe_values(
            &mut builder.qualification_effects,
            &row.qualification_effects,
        );
        builder
            .stop_placement_status
            .insert(format!("member-role:{}", row.member_role));

        if let Some(pavement) = pavement_by_member.get(&national_segment_member_key(
            &row.segment_bundle_id,
            &row.national_segment_id,
        )) {
            insert_semicolon_values(
                &mut builder.source_artifacts,
                "data/tier-pavement-docket.csv",
            );
            builder
                .stop_placement_status
                .insert(pavement.pavement_status.clone());
            insert_pipe_values(
                &mut builder.qualification_effects,
                &pavement.qualification_effects,
            );
        } else {
            builder
                .stop_placement_status
                .insert("pavement-docket-missing".to_string());
        }
    }

    builders
        .into_values()
        .map(|builder| {
            let validation_status = if builder
                .validation_statuses
                .iter()
                .all(|status| status == "pass")
            {
                "pass"
            } else {
                "review"
            };
            let registry_action = national_segment_registry_action(
                &builder.board_layers,
                &builder.stop_placement_status,
                &builder.evidence_state_scope,
                &builder.geometry_state_scope,
            );
            let evidence_state_scope = join_string_set(&builder.evidence_state_scope);
            let geometry_state_scope = join_string_set(&builder.geometry_state_scope);
            let state_scope = if geometry_state_scope.is_empty() {
                evidence_state_scope.clone()
            } else {
                geometry_state_scope.clone()
            };
            let route_label = normalise_designation(&builder.route);
            NationalSegmentRegistryRow {
                member_segment_ids: builder.national_segment_id.clone(),
                bundle_role: builder.bundle_role,
                national_segment_id: builder.national_segment_id,
                segment_bundle_id: builder.segment_bundle_id,
                stitch_group_id: builder.stitch_group_id,
                current_zone_id: builder.zone_id.clone(),
                current_tier: builder.current_tier,
                route_label: route_label.clone(),
                zone_id: builder.zone_id,
                route: builder.route,
                state_scope,
                evidence_state_scope,
                geometry_state_scope,
                segment_aliases: join_string_set(&builder.segment_aliases),
                bundle_aliases: join_string_set(&builder.bundle_aliases),
                board_layers: join_string_set(&builder.board_layers),
                source_artifacts: join_string_set(&builder.source_artifacts),
                stop_placement_status: join_string_set(&builder.stop_placement_status),
                registry_action: registry_action.to_string(),
                qualification_effects: join_pipe_set(&builder.qualification_effects),
                validation_status: validation_status.to_string(),
            }
        })
        .collect()
}

