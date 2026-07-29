//! Helper `tier_region_workload_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_region_workload_rows(
    graph: &route_network::HighwayGraph,
    tier: &str,
    routes: &[String],
    parent_routes: &[String],
    graph_kind: route_network::ServiceGraphKind,
    requested_regions: usize,
) -> Result<Vec<TierRegionWorkloadRow>> {
    if routes.is_empty() {
        anyhow::bail!("{tier} has no routes in tier table");
    }
    if requested_regions == 0 {
        anyhow::bail!("requested regions must be >= 1");
    }
    if requested_regions > routes.len() {
        anyhow::bail!(
            "requested regions ({requested_regions}) cannot exceed route count ({})",
            routes.len()
        );
    }

    let (mut adjacency, contact_counts) = dual_route_adjacency(graph, routes, parent_routes);
    let (component_ids, component_count) = connected_components(&adjacency);
    let component_sizes = component_sizes(&component_ids, component_count);
    let component_status = if component_count <= 1 {
        "connected".to_string()
    } else {
        bridge_components(&mut adjacency, &component_ids, component_count);
        format!("component-bridged:{component_count}")
    };
    let weights = routes
        .iter()
        .map(|route| route_region_weight(graph.route_miles(route)))
        .collect::<Vec<_>>();
    let input =
        route_network::ServiceGraphPartitionInput::new(graph_kind, adjacency, weights.clone())
            .map_err(|err| anyhow::anyhow!(err))?;
    let assignment =
        route_network::partition_service_graph_input_metis(&input, requested_regions, Some(10))
            .map_err(|err| anyhow::anyhow!(err))?;
    validate_region_assignment(&assignment.assignment, requested_regions)?;
    let connectivity = route_network::analyze_tier_connectivity(graph, routes, parent_routes)
        .into_iter()
        .map(|row| (row.route.clone(), row))
        .collect::<std::collections::HashMap<_, _>>();

    Ok(routes
        .iter()
        .enumerate()
        .map(|(idx, route)| {
            let connectivity_row = connectivity
                .get(route)
                .unwrap_or_else(|| panic!("missing connectivity row for {route}"));
            let repair = tier_region_repair_action(
                &connectivity_row.classification,
                contact_counts[idx],
                component_sizes[component_ids[idx]],
            );
            TierRegionWorkloadRow {
                tier: tier.to_string(),
                graph_kind: assignment.graph_kind.mode_name().to_string(),
                split_objective: "route-mile-workload".to_string(),
                requested_regions,
                region_id: assignment.assignment[idx],
                route: route.clone(),
                node_class: connectivity_row.classification.as_str().to_string(),
                route_weight: weights[idx],
                route_miles: graph.route_miles(route),
                t1_node_count: connectivity_row.t1_node_count,
                parent_trunk_count: connectivity_row.t1_routes.len(),
                parent_trunks: connectivity_row.t1_routes.join(";"),
                contact_route_count: contact_counts[idx],
                component_id: component_ids[idx],
                component_route_count: component_sizes[component_ids[idx]],
                component_status: component_status.clone(),
                repair_action: repair.0.to_string(),
                repair_basis: repair.1.to_string(),
                validation_status: if component_status == "connected" {
                    "pass"
                } else {
                    "review"
                }
                .to_string(),
            }
        })
        .collect())
}

