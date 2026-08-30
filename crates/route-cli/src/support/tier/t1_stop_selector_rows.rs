//! Helper `t1_stop_selector_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_stop_selector_rows(
    selector_rows: &[T1LineSelectorInputRow],
    stop_rows: &[StopCandidateRow],
    target_regions: usize,
) -> Result<Vec<T1StopSelectorRow>> {
    if target_regions == 0 {
        anyhow::bail!("target_regions must be >= 1");
    }
    let mut rows = Vec::new();
    for selector in selector_rows.iter().filter(|row| row.selected) {
        let route = normalise_designation(&selector.route);
        let selected_ids = selector
            .selected_stops
            .split(';')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .collect::<std::collections::BTreeSet<_>>();
        let mut route_stops = stop_rows
            .iter()
            .filter(|stop| selected_ids.contains(stop.stop_id.as_str()))
            .filter(|stop| {
                stop_candidate_routes(stop)
                    .iter()
                    .any(|candidate| candidate == &route)
            })
            .collect::<Vec<_>>();
        sort_stops_for_route(&mut route_stops);
        if route_stops.is_empty() {
            anyhow::bail!("{route}: selected route has no stop candidates");
        }
        let region_count = target_regions.min(route_stops.len());
        let weights = route_stops
            .iter()
            .map(|stop| i32::from(stop_candidate_selector_score(stop).max(1)))
            .collect::<Vec<_>>();
        let input = route_network::LinearRouteSplitInput::with_weights(
            route_network::LinearRouteSplitObjective::HybridService,
            weights.clone(),
        )
        .map_err(|err| anyhow::anyhow!(err))?;
        let regions = route_network::linear_route_stop_regions_with_input(&input, region_count)
            .map_err(|err| anyhow::anyhow!("{route}: {err}"))?;
        let split_stops = route_network::linear_route_split_stops_with_input(&input, region_count)
            .map_err(|err| anyhow::anyhow!("{route}: {err}"))?;
        let mut region_by_stop = vec![0usize; route_stops.len()];
        for region in regions {
            for idx in region.start_stop_index..=region.end_stop_index {
                region_by_stop[idx] = region.region_index;
            }
        }
        let boundary_after = split_stops
            .iter()
            .map(|split| split.before_stop_index)
            .collect::<std::collections::BTreeSet<_>>();
        for (idx, stop) in route_stops.iter().enumerate() {
            rows.push(T1StopSelectorRow {
                route: route.clone(),
                stop_sequence: idx + 1,
                stop_id: stop.stop_id.clone(),
                stop_name: stop.name.clone(),
                requested_class: stop.requested_class.clone(),
                selector_weight: weights[idx],
                split_objective: route_network::LinearRouteSplitObjective::HybridService
                    .mode_name()
                    .to_string(),
                target_regions: region_count,
                metis_region: region_by_stop[idx],
                boundary_after: boundary_after.contains(&idx),
                evidence_status: stop.evidence_status.clone(),
                validation_status: "pass".to_string(),
            });
        }
    }
    Ok(rows)
}
