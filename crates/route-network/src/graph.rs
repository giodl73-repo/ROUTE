use geo_types::{Coord, LineString};
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use std::collections::HashMap;

/// A node in the highway graph — an intersection, terminus, or state-line crossing.
#[derive(Debug, Clone)]
pub struct HighwayNode {
    pub id: u64,
    /// Coordinate in EPSG:4269 (NAD83 geographic)
    pub coord: Coord<f64>,
    /// True if this node is an interchange (connects ≥2 routes)
    pub is_interchange: bool,
}

/// An edge in the highway graph — one homogeneous NHS segment.
#[derive(Debug, Clone)]
pub struct HighwayEdge {
    pub id: u64,
    /// Normalised route identifier, e.g. "I80", "I95"
    pub route_id: String,
    pub state: String,
    pub geometry: LineString<f64>,
    pub length_miles: f64,
    pub lane_count: Option<u8>,
    // Joined from HPMS (None = join failed)
    pub aadt: Option<u32>,
    /// Truck proportion 0.0–1.0
    pub pct_truck: Option<f32>,
    pub iri: Option<f32>,
    pub tti: Option<f32>,
    pub pti: Option<f32>,
}

/// The national highway graph.
/// Uses a directed graph so betweenness centrality captures freight flow asymmetry.
pub struct HighwayGraph {
    /// petgraph directed graph; nodes = intersections, edges = NHS segments
    pub graph: DiGraph<HighwayNode, HighwayEdge>,
    /// Maps route_id → all edge indices for that route
    pub route_index: HashMap<String, Vec<EdgeIndex>>,
    /// Maps route_id → [start_node, end_node] (termini)
    pub terminus_index: HashMap<String, [NodeIndex; 2]>,
    /// Betweenness centrality per edge.
    /// None until `route score-all` completes the full national graph.
    pub edge_betweenness: Option<HashMap<EdgeIndex, f64>>,
}

impl HighwayGraph {
    pub fn new() -> Self {
        HighwayGraph {
            graph: DiGraph::new(),
            route_index: HashMap::new(),
            terminus_index: HashMap::new(),
            edge_betweenness: None,
        }
    }

    /// All edge indices belonging to a route. Empty vec if route not found.
    pub fn route_edges(&self, route_id: &str) -> &[EdgeIndex] {
        self.route_index.get(route_id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Total route length in miles.
    pub fn route_miles(&self, route_id: &str) -> f64 {
        self.route_edges(route_id)
            .iter()
            .map(|&ei| self.graph[ei].length_miles)
            .sum()
    }

    /// All unique route IDs in the graph.
    pub fn route_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.route_index.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Interstate route IDs only (route_id starts with "I").
    pub fn interstate_ids(&self) -> Vec<String> {
        self.route_ids()
            .into_iter()
            .filter(|id| id.starts_with('I'))
            .collect()
    }

    /// Build report: print join failure counts per source.
    pub fn print_build_report(&self, join_report: &JoinReport) {
        println!("=== route build report ===");
        println!("  nodes:              {}", self.graph.node_count());
        println!("  edges:              {}", self.graph.edge_count());
        println!("  routes:             {}", self.route_index.len());
        println!("  interstates:        {}", self.interstate_ids().len());
        println!("  HPMS join failures: {}", join_report.hpms_failures);
        println!("  NBI join failures:  {}", join_report.nbi_failures);
        println!("  FAF5 zone coverage: {:.0}% of routes", join_report.faf5_coverage_pct);
        if !join_report.data_sparse_routes.is_empty() {
            println!("  data-sparse routes ({} primary fields None):", join_report.data_sparse_threshold);
            for r in &join_report.data_sparse_routes {
                println!("    {r}");
            }
        }
    }
}

impl Default for HighwayGraph {
    fn default() -> Self { Self::new() }
}

/// Summary of join failures from `route build`.
#[derive(Debug, Default)]
pub struct JoinReport {
    pub hpms_failures: usize,
    pub nbi_failures: usize,
    pub faf5_coverage_pct: f64,
    pub data_sparse_routes: Vec<String>,
    pub data_sparse_threshold: usize,
}
