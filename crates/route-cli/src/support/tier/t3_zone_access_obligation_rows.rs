//! Helper `t3_zone_access_obligation_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_access_obligation_rows(
    intake_rows: &[T3T4PressureIntakeRow],
    atlas_rows: &[MapAtlasRow],
) -> Vec<T3ZoneAccessObligationRow> {
    #[derive(Default)]
    struct ObligationAggregate {
        routes: std::collections::BTreeSet<String>,
        intake_classes: std::collections::BTreeSet<String>,
    }

    let t3_maps = t3_zone_map_ids(atlas_rows);
    let mut aggregates = std::collections::BTreeMap::<(String, String), ObligationAggregate>::new();

    for row in intake_rows {
        let Some((zone_id, _zone_name)) = t3_zone_for_route(&row.route) else {
            continue;
        };
        if !t3_maps.contains(zone_id) {
            continue;
        }
        let obligation_class = t3_obligation_class_for_intake(&row.intake_class);
        let aggregate = aggregates
            .entry((zone_id.to_string(), obligation_class.to_string()))
            .or_default();
        aggregate.routes.insert(row.route.clone());
        aggregate.intake_classes.insert(row.intake_class.clone());
    }

    let mut rows = aggregates
        .into_iter()
        .filter_map(|((zone_id, obligation_class), aggregate)| {
            let (_zone_id, zone_name) = t3_zone_catalog_entry(&zone_id)?;
            let (access_target, horizon, next_artifact, optimizer_effect) =
                t3_zone_obligation_contract(&obligation_class);
            Some(T3ZoneAccessObligationRow {
                zone_id: zone_id.clone(),
                zone_name: zone_name.to_string(),
                obligation_class: obligation_class.clone(),
                access_target: access_target.to_string(),
                promise_horizon_hours: horizon,
                source_route_count: aggregate.routes.len(),
                candidate_routes: aggregate
                    .routes
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(";"),
                source_intake_classes: aggregate
                    .intake_classes
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(";"),
                map_id: zone_id,
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            })
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.zone_id
            .cmp(&b.zone_id)
            .then_with(|| a.promise_horizon_hours.cmp(&b.promise_horizon_hours))
            .then_with(|| a.obligation_class.cmp(&b.obligation_class))
    });
    rows
}

