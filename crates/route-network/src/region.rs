use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use thiserror::Error;

/// Region split/composition strategy for tier optimizer workloads.
///
/// This mirrors BISECT's split-strategy layer, but the units are service
/// regions and tier treatments rather than electoral districts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionStrategy {
    StandardBisection,
    PrimeFactorSpine,
    CapacityClustering,
    FlowConstruction,
    Spectral,
    Regionalization,
}

impl RegionStrategy {
    pub fn mode_name(self) -> &'static str {
        match self {
            Self::StandardBisection => "standard-bisection",
            Self::PrimeFactorSpine => "prime-factor-spine",
            Self::CapacityClustering => "capacity-clustering",
            Self::FlowConstruction => "flow-construction",
            Self::Spectral => "spectral",
            Self::Regionalization => "regionalization",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchCompositor {
    Single,
    Multi {
        candidates: usize,
    },
    ConvergenceSweep {
        threshold: u32,
    },
    /// Percentile encoded as basis points: 0 = min, 5000 = median, 10000 = max.
    Percentile {
        percentile_bps: u16,
        candidates: usize,
    },
    ParetoFrontier {
        selected_index: usize,
    },
}

impl SearchCompositor {
    pub fn candidate_count(self) -> usize {
        match self {
            Self::Single => 1,
            Self::Multi { candidates } => candidates,
            Self::ConvergenceSweep { threshold } => threshold as usize,
            Self::Percentile { candidates, .. } => candidates,
            Self::ParetoFrontier { .. } => 1,
        }
    }
}

impl Default for SearchCompositor {
    fn default() -> Self {
        Self::Single
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceWeightSpec {
    pub sla_promise: bool,
    pub freight_market: bool,
    pub top_city_pairs: bool,
    pub intermodal_access: bool,
    pub resilience: bool,
    pub stop_spacing: bool,
    pub evidence_penalty: bool,
    pub duplicate_penalty: bool,
}

impl Default for ServiceWeightSpec {
    fn default() -> Self {
        Self {
            sla_promise: true,
            freight_market: true,
            top_city_pairs: true,
            intermodal_access: true,
            resilience: true,
            stop_spacing: true,
            evidence_penalty: true,
            duplicate_penalty: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TierOptimizerConfig {
    pub region_strategy: RegionStrategy,
    pub weights: ServiceWeightSpec,
    pub search: SearchCompositor,
    pub mode_label: Option<String>,
}

impl TierOptimizerConfig {
    pub fn recursive_regionalization() -> Self {
        Self {
            region_strategy: RegionStrategy::PrimeFactorSpine,
            weights: ServiceWeightSpec::default(),
            search: SearchCompositor::Single,
            mode_label: Some("recursive-regionalization".to_string()),
        }
    }

    pub fn mode_name(&self) -> &str {
        self.mode_label
            .as_deref()
            .unwrap_or_else(|| self.region_strategy.mode_name())
    }
}

/// A single non-leaf service-region workload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionWorkloadNode {
    pub target_treatments: usize,
    pub left_treatments: usize,
    pub right_treatments: usize,
    pub depth: usize,
    pub path: String,
}

/// Pure split schedule for recursive tier treatment.
///
/// This is intentionally geography-free. It answers only "how many regional
/// treatment leaves should each child carry?" Later passes bind leaves to
/// actual T1/T2/T3/T4 geography and service obligations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionWorkloadTree {
    pub target_treatments: usize,
    pub max_depth: usize,
    pub nodes: Vec<RegionWorkloadNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearRouteStopRegion {
    pub region_index: usize,
    pub start_stop_index: usize,
    pub end_stop_index: usize,
    pub stop_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearRouteSplitStop {
    pub split_index: usize,
    pub before_stop_index: usize,
    pub after_stop_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinearRouteSplitObjective {
    EqualStops,
    EqualDistance,
    EqualFreight,
    EqualPopulation,
    HybridService,
}

impl LinearRouteSplitObjective {
    pub fn mode_name(self) -> &'static str {
        match self {
            Self::EqualStops => "equal-stops",
            Self::EqualDistance => "equal-distance",
            Self::EqualFreight => "equal-freight",
            Self::EqualPopulation => "equal-population",
            Self::HybridService => "hybrid-service",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceGraphKind {
    /// Stops/cities/interchanges are vertices; route segments are edges.
    PrimalStopGraph,
    /// Routes/corridors are vertices; shared stops/transfers/overlaps are edges.
    DualRouteGraph,
}

impl ServiceGraphKind {
    pub fn mode_name(self) -> &'static str {
        match self {
            Self::PrimalStopGraph => "primal-stop-graph",
            Self::DualRouteGraph => "dual-route-graph",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceGraphPartitionInput {
    pub graph_kind: ServiceGraphKind,
    pub adjacency: Vec<Vec<usize>>,
    pub vertex_weights: Vec<i32>,
}

impl ServiceGraphPartitionInput {
    pub fn new(
        graph_kind: ServiceGraphKind,
        adjacency: Vec<Vec<usize>>,
        vertex_weights: Vec<i32>,
    ) -> Result<Self, RegionPartitionError> {
        validate_partition_inputs(&adjacency, Some(&vertex_weights), 1)?;
        Ok(Self {
            graph_kind,
            adjacency,
            vertex_weights,
        })
    }

    pub fn primal_stop_graph(
        adjacency: Vec<Vec<usize>>,
        vertex_weights: Vec<i32>,
    ) -> Result<Self, RegionPartitionError> {
        Self::new(ServiceGraphKind::PrimalStopGraph, adjacency, vertex_weights)
    }

    pub fn dual_route_graph(
        adjacency: Vec<Vec<usize>>,
        vertex_weights: Vec<i32>,
    ) -> Result<Self, RegionPartitionError> {
        Self::new(ServiceGraphKind::DualRouteGraph, adjacency, vertex_weights)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearRouteSplitInput {
    pub objective: LinearRouteSplitObjective,
    pub stop_weights: Vec<i32>,
}

impl LinearRouteSplitInput {
    pub fn equal_stops(stop_count: usize) -> Self {
        Self {
            objective: LinearRouteSplitObjective::EqualStops,
            stop_weights: vec![1; stop_count],
        }
    }

    pub fn with_weights(
        objective: LinearRouteSplitObjective,
        stop_weights: Vec<i32>,
    ) -> Result<Self, RegionPartitionError> {
        if stop_weights.is_empty() {
            return Err(RegionPartitionError::InvalidInput(
                "stop_weights must not be empty".to_string(),
            ));
        }
        if stop_weights.iter().any(|&weight| weight <= 0) {
            return Err(RegionPartitionError::InvalidInput(
                "stop_weights must be positive".to_string(),
            ));
        }
        Ok(Self {
            objective,
            stop_weights,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetisRegionAssignment {
    pub graph_kind: ServiceGraphKind,
    pub parts: usize,
    pub assignment: Vec<usize>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RegionPartitionError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("metis partition failed: {0}")]
    Metis(String),
}

impl RegionWorkloadTree {
    pub fn from_target_count(target_treatments: usize) -> Self {
        assert!(target_treatments >= 1, "target_treatments must be >= 1");
        let max_depth = max_depth_for_target_count(target_treatments);
        let mut nodes = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((target_treatments, 0usize, String::new()));

        while let Some((target, depth, path)) = queue.pop_front() {
            if target <= 1 {
                continue;
            }
            let left = target / 2;
            let right = target - left;
            nodes.push(RegionWorkloadNode {
                target_treatments: target,
                left_treatments: left,
                right_treatments: right,
                depth,
                path: path.clone(),
            });
            queue.push_back((left, depth + 1, format!("{path}0")));
            queue.push_back((right, depth + 1, format!("{path}1")));
        }

        Self {
            target_treatments,
            max_depth,
            nodes,
        }
    }

    pub fn nodes_at_depth(&self, depth: usize) -> Vec<&RegionWorkloadNode> {
        self.nodes
            .iter()
            .filter(|node| node.depth == depth)
            .collect()
    }

    pub fn splits_per_depth(&self) -> Vec<usize> {
        let mut counts = vec![0usize; self.max_depth + 1];
        for node in &self.nodes {
            counts[node.depth] += 1;
        }
        counts
    }

    pub fn total_splits(&self) -> usize {
        self.nodes.len()
    }
}

pub fn max_depth_for_target_count(target_treatments: usize) -> usize {
    if target_treatments <= 1 {
        return 0;
    }
    let mut depth = 0usize;
    let mut current = 1usize;
    while current < target_treatments {
        current *= 2;
        depth += 1;
    }
    depth
}

/// Split an ordered route stop chain into N contiguous stop regions.
///
/// This is the corridor analogue to the region workload tree: the country can
/// be recursively split into service regions, while a single linear route can
/// be split into contiguous stop workloads for spacing, relay, or schematic
/// review.
pub fn linear_route_stop_regions(
    stop_count: usize,
    region_count: usize,
) -> Result<Vec<LinearRouteStopRegion>, RegionPartitionError> {
    linear_route_stop_regions_with_input(
        &LinearRouteSplitInput::equal_stops(stop_count),
        region_count,
    )
}

pub fn linear_route_stop_regions_with_input(
    input: &LinearRouteSplitInput,
    region_count: usize,
) -> Result<Vec<LinearRouteStopRegion>, RegionPartitionError> {
    let stop_count = input.stop_weights.len();
    if stop_count == 0 {
        return Err(RegionPartitionError::InvalidInput(
            "stop_count must be >= 1".to_string(),
        ));
    }
    if region_count == 0 {
        return Err(RegionPartitionError::InvalidInput(
            "region_count must be >= 1".to_string(),
        ));
    }
    if region_count > stop_count {
        return Err(RegionPartitionError::InvalidInput(
            "region_count cannot exceed stop_count".to_string(),
        ));
    }
    let mut adjacency = vec![Vec::<usize>::new(); stop_count];
    for idx in 0..stop_count {
        if idx > 0 {
            adjacency[idx].push(idx - 1);
        }
        if idx + 1 < stop_count {
            adjacency[idx].push(idx + 1);
        }
    }
    let partition = partition_service_graph_metis(
        &adjacency,
        Some(&input.stop_weights),
        region_count,
        Some(0),
    )?;
    contiguous_stop_regions_from_assignment(&partition.assignment, region_count)
}

/// Split an ordered route stop chain into K METIS regions and return the K-1
/// boundary stop pairs where adjacent regions meet.
pub fn linear_route_split_stops(
    stop_count: usize,
    region_count: usize,
) -> Result<Vec<LinearRouteSplitStop>, RegionPartitionError> {
    let regions = linear_route_stop_regions(stop_count, region_count)?;
    split_stops_from_regions(&regions)
}

pub fn linear_route_split_stops_with_input(
    input: &LinearRouteSplitInput,
    region_count: usize,
) -> Result<Vec<LinearRouteSplitStop>, RegionPartitionError> {
    let regions = linear_route_stop_regions_with_input(input, region_count)?;
    split_stops_from_regions(&regions)
}

fn split_stops_from_regions(
    regions: &[LinearRouteStopRegion],
) -> Result<Vec<LinearRouteSplitStop>, RegionPartitionError> {
    Ok(regions
        .windows(2)
        .enumerate()
        .map(|(split_index, pair)| LinearRouteSplitStop {
            split_index,
            before_stop_index: pair[0].end_stop_index,
            after_stop_index: pair[1].start_stop_index,
        })
        .collect())
}

/// Partition an undirected service graph into region assignments using
/// `metis-core`, the pure-Rust METIS backend used by BISECT.
pub fn partition_service_graph_metis(
    adjacency: &[Vec<usize>],
    vertex_weights: Option<&[i32]>,
    parts: usize,
    seed: Option<u64>,
) -> Result<MetisRegionAssignment, RegionPartitionError> {
    partition_service_graph_kind_metis(
        ServiceGraphKind::PrimalStopGraph,
        adjacency,
        vertex_weights,
        parts,
        seed,
    )
}

pub fn partition_service_graph_input_metis(
    input: &ServiceGraphPartitionInput,
    parts: usize,
    seed: Option<u64>,
) -> Result<MetisRegionAssignment, RegionPartitionError> {
    partition_service_graph_kind_metis(
        input.graph_kind,
        &input.adjacency,
        Some(&input.vertex_weights),
        parts,
        seed,
    )
}

pub fn partition_service_graph_kind_metis(
    graph_kind: ServiceGraphKind,
    adjacency: &[Vec<usize>],
    vertex_weights: Option<&[i32]>,
    parts: usize,
    seed: Option<u64>,
) -> Result<MetisRegionAssignment, RegionPartitionError> {
    validate_partition_inputs(adjacency, vertex_weights, parts)?;
    if parts == 1 {
        return Ok(MetisRegionAssignment {
            graph_kind,
            parts,
            assignment: vec![0; adjacency.len()],
        });
    }

    let (xadj, adjncy) = csr_from_adjacency(adjacency)?;
    let weights = vertex_weights
        .map(|weights| weights.to_vec())
        .unwrap_or_else(|| vec![1; adjacency.len()]);
    let mut params = metis_core::MetisParams::kway()
        .with_ufactor(50)
        .with_niter(100)
        .with_coarsen_to(20);
    if let Some(seed) = seed {
        params = params.with_seed(seed);
    }
    let graph = metis_core::CsrGraph::new(xadj, adjncy, 1, weights, None)
        .map_err(|err| RegionPartitionError::Metis(format!("csr graph: {err}")))?;
    let partitioner = metis_core::MetisPartitioner::with_params(params, parts as u32);
    let result = metis_core::Partitioner::split(&partitioner, &graph, parts as u32, seed)
        .map_err(|err| RegionPartitionError::Metis(format!("k={parts}: {err}")))?;
    Ok(MetisRegionAssignment {
        graph_kind,
        parts,
        assignment: result
            .into_assignment()
            .into_iter()
            .map(|part| part as usize)
            .collect(),
    })
}

fn validate_partition_inputs(
    adjacency: &[Vec<usize>],
    vertex_weights: Option<&[i32]>,
    parts: usize,
) -> Result<(), RegionPartitionError> {
    if adjacency.is_empty() {
        return Err(RegionPartitionError::InvalidInput(
            "adjacency must not be empty".to_string(),
        ));
    }
    if parts == 0 {
        return Err(RegionPartitionError::InvalidInput(
            "parts must be >= 1".to_string(),
        ));
    }
    if parts > adjacency.len() {
        return Err(RegionPartitionError::InvalidInput(
            "parts cannot exceed node count".to_string(),
        ));
    }
    if let Some(weights) = vertex_weights {
        if weights.len() != adjacency.len() {
            return Err(RegionPartitionError::InvalidInput(
                "vertex_weights length must match adjacency length".to_string(),
            ));
        }
        if weights.iter().any(|&weight| weight <= 0) {
            return Err(RegionPartitionError::InvalidInput(
                "vertex_weights must be positive".to_string(),
            ));
        }
    }
    for (node, neighbors) in adjacency.iter().enumerate() {
        for &neighbor in neighbors {
            if neighbor >= adjacency.len() {
                return Err(RegionPartitionError::InvalidInput(format!(
                    "neighbor index {neighbor} from node {node} is out of bounds"
                )));
            }
        }
    }
    Ok(())
}

fn csr_from_adjacency(
    adjacency: &[Vec<usize>],
) -> Result<(Vec<u32>, Vec<u32>), RegionPartitionError> {
    let mut xadj = Vec::with_capacity(adjacency.len() + 1);
    let mut adjncy = Vec::new();
    xadj.push(0);
    for neighbors in adjacency {
        let mut sorted = neighbors.clone();
        sorted.sort_unstable();
        sorted.dedup();
        for neighbor in sorted {
            adjncy.push(u32::try_from(neighbor).map_err(|_| {
                RegionPartitionError::InvalidInput("graph too large for metis-core".to_string())
            })?);
        }
        xadj.push(u32::try_from(adjncy.len()).map_err(|_| {
            RegionPartitionError::InvalidInput("graph too large for metis-core".to_string())
        })?);
    }
    Ok((xadj, adjncy))
}

fn contiguous_stop_regions_from_assignment(
    assignment: &[usize],
    region_count: usize,
) -> Result<Vec<LinearRouteStopRegion>, RegionPartitionError> {
    let mut regions = Vec::with_capacity(region_count);
    for region_index in 0..region_count {
        let indices = assignment
            .iter()
            .enumerate()
            .filter_map(|(idx, &part)| (part == region_index).then_some(idx))
            .collect::<Vec<_>>();
        if indices.is_empty() {
            return Err(RegionPartitionError::Metis(format!(
                "metis produced empty linear stop region {region_index}"
            )));
        }
        let start = *indices.first().unwrap();
        let end = *indices.last().unwrap();
        if end - start + 1 != indices.len() {
            return Err(RegionPartitionError::Metis(format!(
                "metis produced non-contiguous linear stop region {region_index}"
            )));
        }
        regions.push(LinearRouteStopRegion {
            region_index,
            start_stop_index: start,
            end_stop_index: end,
            stop_count: indices.len(),
        });
    }
    regions.sort_by_key(|region| region.start_stop_index);
    for (idx, region) in regions.iter_mut().enumerate() {
        region.region_index = idx;
    }
    Ok(regions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_workload_depth_matches_bisection_schedule() {
        assert_eq!(max_depth_for_target_count(1), 0);
        assert_eq!(max_depth_for_target_count(2), 1);
        assert_eq!(max_depth_for_target_count(3), 2);
        assert_eq!(max_depth_for_target_count(7), 3);
        assert_eq!(max_depth_for_target_count(11), 4);
    }

    #[test]
    fn region_workload_total_splits_equal_leaves_minus_one() {
        for target in [2, 3, 4, 7, 8, 11] {
            let tree = RegionWorkloadTree::from_target_count(target);
            assert_eq!(tree.total_splits(), target - 1, "target={target}");
        }
    }

    #[test]
    fn region_workload_uses_stable_bfs_paths() {
        let tree = RegionWorkloadTree::from_target_count(7);
        let d0 = tree.nodes_at_depth(0);
        assert_eq!(d0.len(), 1);
        assert_eq!(d0[0].path, "");
        assert_eq!((d0[0].left_treatments, d0[0].right_treatments), (3, 4));

        let d1 = tree.nodes_at_depth(1);
        assert_eq!(
            d1.iter().map(|node| node.path.as_str()).collect::<Vec<_>>(),
            vec!["0", "1"]
        );
        assert_eq!(
            d1.iter()
                .map(|node| (node.left_treatments, node.right_treatments))
                .collect::<Vec<_>>(),
            vec![(1, 2), (2, 2)]
        );
    }

    #[test]
    fn optimizer_config_preserves_mode_label_and_independent_layers() {
        let config = TierOptimizerConfig::recursive_regionalization();
        assert_eq!(config.mode_name(), "recursive-regionalization");
        assert_eq!(config.region_strategy, RegionStrategy::PrimeFactorSpine);
        assert_eq!(config.search.candidate_count(), 1);
        assert!(config.weights.sla_promise);
    }

    #[test]
    fn linear_route_stop_regions_are_contiguous_and_balanced() {
        let regions = linear_route_stop_regions(11, 4).unwrap();
        assert_eq!(
            regions
                .iter()
                .map(|region| region.stop_count)
                .sum::<usize>(),
            11
        );
        assert_eq!(regions.first().unwrap().start_stop_index, 0);
        assert_eq!(regions.last().unwrap().end_stop_index, 10);
        for pair in regions.windows(2) {
            assert_eq!(pair[0].end_stop_index + 1, pair[1].start_stop_index);
        }
        assert!(regions.iter().all(|region| region.stop_count > 0));
    }

    #[test]
    fn linear_route_split_stops_are_k_minus_one_boundaries() {
        let splits = linear_route_split_stops(11, 4).unwrap();
        assert_eq!(splits.len(), 3);
        assert_eq!(
            splits
                .iter()
                .map(|split| split.after_stop_index - split.before_stop_index)
                .collect::<Vec<_>>(),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn linear_route_split_objective_changes_metis_weights() {
        let input = LinearRouteSplitInput::with_weights(
            LinearRouteSplitObjective::EqualFreight,
            vec![1, 1, 10, 10, 10, 1, 1],
        )
        .unwrap();
        let regions = linear_route_stop_regions_with_input(&input, 2).unwrap();
        assert_eq!(input.objective.mode_name(), "equal-freight");
        assert_eq!(regions.len(), 2);
        assert_eq!(regions.first().unwrap().start_stop_index, 0);
        assert_eq!(regions.last().unwrap().end_stop_index, 6);
    }

    #[test]
    fn metis_partitions_service_graph_fixture() {
        let adjacency = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let result = partition_service_graph_metis(&adjacency, Some(&[1, 1, 1, 1]), 2, Some(7))
            .expect("metis partition");
        assert_eq!(result.graph_kind, ServiceGraphKind::PrimalStopGraph);
        assert_eq!(result.parts, 2);
        assert_eq!(result.assignment.len(), 4);
        assert!(result.assignment.iter().all(|&part| part < 2));
        assert_eq!(
            result
                .assignment
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            2
        );
    }

    #[test]
    fn metis_partitions_dual_route_graph_fixture() {
        let input = ServiceGraphPartitionInput::dual_route_graph(
            vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
            vec![10, 4, 4, 10],
        )
        .unwrap();
        let result = partition_service_graph_input_metis(&input, 2, Some(11)).unwrap();
        assert_eq!(result.graph_kind, ServiceGraphKind::DualRouteGraph);
        assert_eq!(result.assignment.len(), 4);
        assert!(result.assignment.iter().all(|&part| part < 2));
    }
}
