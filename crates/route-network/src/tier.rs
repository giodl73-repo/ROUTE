use crate::graph::HighwayGraph;
use petgraph::graph::NodeIndex;
use std::collections::{BTreeSet, HashSet};

const TIER_ROUTE_CONTACT_TOLERANCE_MILES: f64 = 1.0;

pub const T1_SCORE_THRESHOLD: f64 = 70.0;
pub const T2_SCORE_THRESHOLD: f64 = 50.0;
pub const T3_SCORE_THRESHOLD: f64 = 30.0;

pub const T1_BACKBONE_ROUTES: &[&str] = &["I5", "I10", "I35", "I40", "I75", "I80", "I90", "I95"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RouteTier {
    T1,
    T2,
    T3,
    T4,
}

impl RouteTier {
    pub fn from_score(score: f64) -> Self {
        if score >= T1_SCORE_THRESHOLD {
            Self::T1
        } else if score >= T2_SCORE_THRESHOLD {
            Self::T2
        } else if score >= T3_SCORE_THRESHOLD {
            Self::T3
        } else {
            Self::T4
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::T1 => "T1",
            Self::T2 => "T2",
            Self::T3 => "T3",
            Self::T4 => "T4",
        }
    }

    pub fn is_backbone(self) -> bool {
        self == Self::T1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StopServiceClass {
    S1,
    S2,
    S3,
    S4,
    S5,
}

impl StopServiceClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::S1 => "S1",
            Self::S2 => "S2",
            Self::S3 => "S3",
            Self::S4 => "S4",
            Self::S5 => "S5",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim_end_matches('?').to_ascii_uppercase().as_str() {
            "S1" => Some(Self::S1),
            "S2" => Some(Self::S2),
            "S3" => Some(Self::S3),
            "S4" => Some(Self::S4),
            "S5" => Some(Self::S5),
            _ => None,
        }
    }

    pub fn is_transfer_grade(self) -> bool {
        matches!(self, Self::S1 | Self::S2 | Self::S3)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StopNodeClass {
    NationalTransferHub,
    NationalTerminal,
    TransferHub,
    ServiceStop,
}

impl StopNodeClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NationalTransferHub => "national_transfer_hub",
            Self::NationalTerminal => "national_terminal",
            Self::TransferHub => "transfer_hub",
            Self::ServiceStop => "service_stop",
        }
    }

    pub fn is_system_contact(self) -> bool {
        !matches!(self, Self::ServiceStop)
    }

    pub fn qualifies_for_route_endpoint(self, tier: RouteTier) -> bool {
        match tier {
            RouteTier::T1 => self.is_system_contact(),
            RouteTier::T2 => self.is_system_contact(),
            RouteTier::T3 => true,
            RouteTier::T4 => true,
        }
    }

    pub fn qualifies_for_route_contact(self, tier: RouteTier) -> bool {
        match tier {
            RouteTier::T1 => true,
            RouteTier::T2 => self.is_system_contact(),
            RouteTier::T3 => true,
            RouteTier::T4 => true,
        }
    }
}

pub fn minimum_system_contacts_for_tier(tier: RouteTier) -> usize {
    match tier {
        RouteTier::T1 => 2,
        RouteTier::T2 => 2,
        RouteTier::T3 => 1,
        RouteTier::T4 => 0,
    }
}

pub fn endpoint_rule_label(tier: RouteTier) -> &'static str {
    match tier {
        RouteTier::T1 => "T1 endpoints must be terminals or transfer hubs",
        RouteTier::T2 => "T2 routes must touch at least two system-contact stops",
        RouteTier::T3 => "T3 routes must connect into local T1/T2 context",
        RouteTier::T4 => "T4 routes are local access/spur candidates",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TierNodeClass {
    TrunkConnector,
    ReliefLoop,
    OneEndedFeeder,
    LocalSpur,
    MissingGraphData,
}

impl TierNodeClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TrunkConnector => "trunk_connector",
            Self::ReliefLoop => "relief_loop",
            Self::OneEndedFeeder => "one_ended_feeder",
            Self::LocalSpur => "local_spur",
            Self::MissingGraphData => "missing_graph_data",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TierTouchNode {
    pub node_id: u64,
    pub lon: f64,
    pub lat: f64,
    pub t1_routes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TierConnectivityRow {
    pub route: String,
    pub route_miles: f64,
    pub t1_node_count: usize,
    pub t1_routes: Vec<String>,
    pub touch_nodes: Vec<TierTouchNode>,
    pub classification: TierNodeClass,
}

pub fn analyze_tier_connectivity(
    graph: &HighwayGraph,
    routes: &[String],
    t1_routes: &[String],
) -> Vec<TierConnectivityRow> {
    routes
        .iter()
        .map(|route| analyze_route_connectivity(graph, route, t1_routes))
        .collect()
}

pub fn tier_connectivity_gate_failures(rows: &[TierConnectivityRow]) -> Vec<&TierConnectivityRow> {
    rows.iter()
        .filter(|row| {
            !matches!(
                row.classification,
                TierNodeClass::TrunkConnector | TierNodeClass::ReliefLoop
            )
        })
        .collect()
}

fn analyze_route_connectivity(
    graph: &HighwayGraph,
    route: &str,
    t1_routes: &[String],
) -> TierConnectivityRow {
    let route_edges = graph.route_edges(route);
    if route_edges.is_empty() {
        return TierConnectivityRow {
            route: route.to_string(),
            route_miles: 0.0,
            t1_node_count: 0,
            t1_routes: Vec::new(),
            touch_nodes: Vec::new(),
            classification: TierNodeClass::MissingGraphData,
        };
    }

    let t1_set = t1_routes.iter().map(String::as_str).collect::<HashSet<_>>();
    let route_node_set = route_nodes(graph, route);
    let mut t1_routes_touched = BTreeSet::new();
    let mut touch_nodes = Vec::new();

    for node in route_node_set {
        let incident_t1 = incident_t1_routes(graph, node, route, &t1_set);
        if incident_t1.is_empty() {
            continue;
        }
        t1_routes_touched.extend(incident_t1.iter().cloned());
        let coord = graph.graph[node].coord;
        touch_nodes.push(TierTouchNode {
            node_id: graph.graph[node].id,
            lon: coord.x,
            lat: coord.y,
            t1_routes: incident_t1,
        });
    }

    touch_nodes.sort_by(|a, b| {
        a.lon
            .total_cmp(&b.lon)
            .then_with(|| a.lat.total_cmp(&b.lat))
            .then_with(|| a.node_id.cmp(&b.node_id))
    });

    let route_miles = graph.route_miles(route);
    let t1_routes = t1_routes_touched.into_iter().collect::<Vec<_>>();
    let classification = classify_route(route_miles, touch_nodes.len(), t1_routes.len());

    TierConnectivityRow {
        route: route.to_string(),
        route_miles,
        t1_node_count: touch_nodes.len(),
        t1_routes,
        touch_nodes,
        classification,
    }
}

fn classify_route(
    route_miles: f64,
    t1_node_count: usize,
    distinct_t1_routes: usize,
) -> TierNodeClass {
    if t1_node_count >= 2 && distinct_t1_routes >= 2 {
        TierNodeClass::TrunkConnector
    } else if t1_node_count >= 2 {
        TierNodeClass::ReliefLoop
    } else if t1_node_count == 0 && route_miles >= 75.0 {
        TierNodeClass::MissingGraphData
    } else if t1_node_count == 1 && route_miles >= 75.0 {
        TierNodeClass::OneEndedFeeder
    } else {
        TierNodeClass::LocalSpur
    }
}

fn route_nodes(graph: &HighwayGraph, route: &str) -> BTreeSet<NodeIndex> {
    graph
        .route_edges(route)
        .iter()
        .filter_map(|&edge| graph.graph.edge_endpoints(edge))
        .flat_map(|(source, target)| [source, target])
        .collect()
}

fn incident_t1_routes(
    graph: &HighwayGraph,
    node: NodeIndex,
    route: &str,
    t1_routes: &HashSet<&str>,
) -> Vec<String> {
    let mut routes = BTreeSet::new();
    for edge in graph.graph.edges(node).chain(
        graph
            .graph
            .edges_directed(node, petgraph::Direction::Incoming),
    ) {
        let route_id = edge.weight().route_id.as_str();
        if route_id != route && t1_routes.contains(route_id) {
            routes.insert(route_id.to_string());
        }
    }
    let node_coord = graph.graph[node].coord;
    for t1_route in t1_routes {
        if *t1_route == route || routes.contains(*t1_route) {
            continue;
        }
        if route_touches_coord(
            graph,
            t1_route,
            node_coord.y,
            node_coord.x,
            TIER_ROUTE_CONTACT_TOLERANCE_MILES,
        ) {
            routes.insert((*t1_route).to_string());
        }
    }
    routes.into_iter().collect()
}

fn route_touches_coord(
    graph: &HighwayGraph,
    route: &str,
    lat: f64,
    lon: f64,
    tolerance_miles: f64,
) -> bool {
    graph.route_edges(route).iter().any(|&edge| {
        graph.graph[edge]
            .geometry
            .0
            .iter()
            .any(|coord| haversine_miles(lat, lon, coord.y, coord.x) <= tolerance_miles)
    })
}

fn haversine_miles(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let radius_miles = 3958.8_f64;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    2.0 * radius_miles * a.sqrt().asin()
}

#[cfg(test)]
mod tests {
    use super::{
        analyze_tier_connectivity, endpoint_rule_label, minimum_system_contacts_for_tier,
        tier_connectivity_gate_failures, RouteTier, StopNodeClass, StopServiceClass, TierNodeClass,
        T1_BACKBONE_ROUTES,
    };
    use crate::graph::{HighwayEdge, HighwayGraph, HighwayNode};
    use geo_types::{coord, LineString};

    fn node(id: u64, x: f64, y: f64) -> HighwayNode {
        HighwayNode {
            id,
            coord: coord! { x: x, y: y },
            is_interchange: true,
        }
    }

    fn edge(id: u64, route_id: &str, miles: f64) -> HighwayEdge {
        edge_at(id, route_id, miles, (0.0, 0.0), (1.0, 1.0))
    }

    fn edge_at(
        id: u64,
        route_id: &str,
        miles: f64,
        start: (f64, f64),
        end: (f64, f64),
    ) -> HighwayEdge {
        HighwayEdge {
            id,
            route_id: route_id.to_string(),
            state: String::new(),
            road_class: route_data::RoadClass::Interstate,
            geometry: LineString::from(vec![start, end]),
            length_miles: miles,
            lane_count: None,
            aadt: None,
            pct_truck: None,
            iri: None,
            tti: None,
            pti: None,
            speed_limit: None,
        }
    }

    #[test]
    fn t2_connector_must_touch_two_distinct_t1_trunks() {
        let mut graph = HighwayGraph::new();
        let west = graph.graph.add_node(node(1, 0.0, 0.0));
        let mid = graph.graph.add_node(node(2, 1.0, 0.0));
        let east = graph.graph.add_node(node(3, 2.0, 0.0));
        let t1a = graph.graph.add_edge(west, mid, edge(1, "I10", 100.0));
        let t2 = graph.graph.add_edge(west, east, edge(2, "I65", 200.0));
        let t1b = graph.graph.add_edge(east, mid, edge(3, "I75", 100.0));
        graph.route_index.insert("I10".to_string(), vec![t1a]);
        graph.route_index.insert("I65".to_string(), vec![t2]);
        graph.route_index.insert("I75".to_string(), vec![t1b]);

        let rows = analyze_tier_connectivity(
            &graph,
            &["I65".to_string()],
            &["I10".to_string(), "I75".to_string()],
        );

        assert_eq!(rows[0].classification, TierNodeClass::TrunkConnector);
        assert_eq!(rows[0].t1_node_count, 2);
        assert!(tier_connectivity_gate_failures(&rows).is_empty());
    }

    #[test]
    fn t2_contact_uses_bounded_geometry_snap_when_nodes_do_not_match() {
        let mut graph = HighwayGraph::new();
        let west = graph.graph.add_node(node(1, -104.991, 39.740));
        let east = graph.graph.add_node(node(2, -95.369, 29.760));
        let t2_west = graph.graph.add_node(node(3, -104.995, 39.744));
        let t2_east = graph.graph.add_node(node(4, -95.373, 29.764));
        let t1a = graph.graph.add_edge(
            west,
            east,
            edge_at(1, "I70", 1000.0, (-104.991, 39.740), (-95.369, 29.760)),
        );
        let t1b = graph.graph.add_edge(
            east,
            west,
            edge_at(2, "I10", 1000.0, (-95.369, 29.760), (-104.991, 39.740)),
        );
        let t2 = graph.graph.add_edge(
            t2_west,
            t2_east,
            edge_at(3, "I25", 400.0, (-104.995, 39.744), (-95.373, 29.764)),
        );
        graph.route_index.insert("I70".to_string(), vec![t1a]);
        graph.route_index.insert("I10".to_string(), vec![t1b]);
        graph.route_index.insert("I25".to_string(), vec![t2]);

        let rows = analyze_tier_connectivity(
            &graph,
            &["I25".to_string()],
            &["I10".to_string(), "I70".to_string()],
        );

        assert_eq!(rows[0].classification, TierNodeClass::TrunkConnector);
        assert_eq!(rows[0].t1_node_count, 2);
    }

    #[test]
    fn short_one_node_spur_is_local_by_default() {
        let mut graph = HighwayGraph::new();
        let hub = graph.graph.add_node(node(1, 0.0, 0.0));
        let out = graph.graph.add_node(node(2, 1.0, 0.0));
        let far = graph.graph.add_node(node(3, 2.0, 0.0));
        let t1 = graph.graph.add_edge(hub, far, edge(1, "I5", 100.0));
        let spur = graph.graph.add_edge(hub, out, edge(2, "I205", 20.0));
        graph.route_index.insert("I5".to_string(), vec![t1]);
        graph.route_index.insert("I205".to_string(), vec![spur]);

        let rows = analyze_tier_connectivity(&graph, &["I205".to_string()], &["I5".to_string()]);

        assert_eq!(rows[0].classification, TierNodeClass::LocalSpur);
        assert_eq!(tier_connectivity_gate_failures(&rows).len(), 1);
    }

    #[test]
    fn route_tier_thresholds_are_canonical() {
        assert_eq!(RouteTier::from_score(70.0), RouteTier::T1);
        assert_eq!(RouteTier::from_score(69.9), RouteTier::T2);
        assert_eq!(RouteTier::from_score(50.0), RouteTier::T2);
        assert_eq!(RouteTier::from_score(49.9), RouteTier::T3);
        assert_eq!(RouteTier::from_score(30.0), RouteTier::T3);
        assert_eq!(RouteTier::from_score(29.9), RouteTier::T4);
    }

    #[test]
    fn t1_backbone_catalog_is_shared_in_normalized_ids() {
        assert_eq!(
            T1_BACKBONE_ROUTES,
            &["I5", "I10", "I35", "I40", "I75", "I80", "I90", "I95"]
        );
    }

    #[test]
    fn stop_service_classes_parse_candidate_suffixes() {
        assert_eq!(StopServiceClass::parse("S1"), Some(StopServiceClass::S1));
        assert_eq!(StopServiceClass::parse("s4?"), Some(StopServiceClass::S4));
        assert!(StopServiceClass::S3.is_transfer_grade());
        assert!(!StopServiceClass::S4.is_transfer_grade());
    }

    #[test]
    fn stop_node_classes_expose_shared_contract_labels() {
        assert_eq!(
            StopNodeClass::NationalTransferHub.as_str(),
            "national_transfer_hub"
        );
        assert!(StopNodeClass::NationalTerminal.is_system_contact());
        assert!(!StopNodeClass::ServiceStop.is_system_contact());
    }

    #[test]
    fn stop_node_classes_encode_route_endpoint_policy() {
        assert_eq!(minimum_system_contacts_for_tier(RouteTier::T1), 2);
        assert_eq!(minimum_system_contacts_for_tier(RouteTier::T2), 2);
        assert_eq!(minimum_system_contacts_for_tier(RouteTier::T3), 1);
        assert!(StopNodeClass::NationalTransferHub.qualifies_for_route_endpoint(RouteTier::T1));
        assert!(StopNodeClass::TransferHub.qualifies_for_route_endpoint(RouteTier::T2));
        assert!(!StopNodeClass::ServiceStop.qualifies_for_route_endpoint(RouteTier::T2));
        assert!(StopNodeClass::ServiceStop.qualifies_for_route_contact(RouteTier::T1));
        assert!(!StopNodeClass::ServiceStop.qualifies_for_route_contact(RouteTier::T2));
        assert!(StopNodeClass::ServiceStop.qualifies_for_route_endpoint(RouteTier::T3));
        assert!(endpoint_rule_label(RouteTier::T2).contains("system-contact"));
    }
}
