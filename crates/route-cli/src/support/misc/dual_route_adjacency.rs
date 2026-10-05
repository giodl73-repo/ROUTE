//! Helper `dual_route_adjacency`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn dual_route_adjacency(
    graph: &route_network::HighwayGraph,
    routes: &[String],
    parent_routes: &[String],
) -> (Vec<Vec<usize>>, Vec<usize>) {
    let route_positions = routes
        .iter()
        .enumerate()
        .map(|(idx, route)| (route.clone(), idx))
        .collect::<std::collections::HashMap<_, _>>();
    let mut node_routes =
        std::collections::HashMap::<usize, std::collections::BTreeSet<usize>>::new();

    for route in routes {
        let Some(&route_idx) = route_positions.get(route) else {
            continue;
        };
        for &edge in graph.route_edges(route) {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                node_routes
                    .entry(source.index())
                    .or_default()
                    .insert(route_idx);
                node_routes
                    .entry(target.index())
                    .or_default()
                    .insert(route_idx);
            }
        }
    }

    let mut adjacency_sets = vec![std::collections::BTreeSet::<usize>::new(); routes.len()];
    for touching_routes in node_routes.values() {
        for &a in touching_routes {
            for &b in touching_routes {
                if a != b {
                    adjacency_sets[a].insert(b);
                }
            }
        }
    }

    let connectivity = route_network::analyze_tier_connectivity(graph, routes, parent_routes);
    let mut parent_route_groups = std::collections::BTreeMap::<String, Vec<usize>>::new();
    for row in connectivity {
        let Some(&route_idx) = route_positions.get(&row.route) else {
            continue;
        };
        for parent_route in row.t1_routes {
            parent_route_groups
                .entry(parent_route)
                .or_default()
                .push(route_idx);
        }
    }
    for touching_routes in parent_route_groups.values() {
        for &a in touching_routes {
            for &b in touching_routes {
                if a != b {
                    adjacency_sets[a].insert(b);
                }
            }
        }
    }
    let contact_counts = adjacency_sets
        .iter()
        .map(std::collections::BTreeSet::len)
        .collect::<Vec<_>>();
    let adjacency = adjacency_sets
        .into_iter()
        .map(|neighbors| neighbors.into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (adjacency, contact_counts)
}
