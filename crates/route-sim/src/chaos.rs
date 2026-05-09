/// Monte Carlo chaos testing.
///
/// Randomly injects incidents (closures, lane reductions, demand spikes)
/// and collects the distribution of outcomes. Answers:
///   - What is the expected annual freight cost of random closures?
///   - Which corridors appear in the worst-outcome scenarios?
///   - Does the diamond interchange reduce variance in closure outcomes?
use crate::assignment::{edge_capacity_vph, wardrop_equilibrium, BprParams};
use crate::demand::DemandMatrix;
use crate::metrics::compute_metrics;
use petgraph::graph::EdgeIndex;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use route_network::HighwayGraph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for a chaos run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosConfig {
    /// Random seed for reproducibility
    pub seed: u64,
    /// Number of Monte Carlo iterations
    pub iterations: usize,
    /// Probability of a random edge closure per iteration
    pub closure_probability: f64,
    /// Mean closure duration (hours), exponentially distributed
    pub mean_duration_hours: f64,
    /// Limit closures to T1 corridors only (if true; false = all interstates)
    pub t1_only: bool,
    /// Frank-Wolfe convergence settings
    pub fw_max_iter: usize,
    pub fw_tolerance: f64,
}

impl Default for ChaosConfig {
    fn default() -> Self {
        ChaosConfig {
            seed: 42,
            iterations: 100,
            closure_probability: 0.05, // 5% chance per edge per iteration
            mean_duration_hours: 4.0,
            t1_only: true,
            fw_max_iter: 20,
            fw_tolerance: 0.01,
        }
    }
}

/// Results from a chaos run.
#[derive(Debug)]
pub struct ChaosResult {
    pub iterations: usize,
    pub mean_freight_cost_delta_m: f64,
    pub p95_freight_cost_delta_m: f64,
    pub max_freight_cost_delta_m: f64,
    /// Corridors appearing in the worst 5% of outcomes
    pub worst_case_corridors: Vec<String>,
    /// Mean PTI across all iterations
    pub mean_network_pti: f64,
    /// Fraction of iterations where V/C > 1.0 on any T1 edge
    pub saturation_fraction: f64,
}

/// Run Monte Carlo chaos simulation.
pub fn run_chaos(g: &HighwayGraph, demand: &DemandMatrix, config: &ChaosConfig) -> ChaosResult {
    let capacities: HashMap<EdgeIndex, f64> = g
        .graph
        .edge_indices()
        .map(|ei| (ei, edge_capacity_vph(g, ei)))
        .collect();

    let bpr = BprParams::default();

    // Baseline equilibrium
    let baseline = wardrop_equilibrium(
        g,
        demand,
        &capacities,
        &bpr,
        config.fw_max_iter,
        config.fw_tolerance,
    );
    let baseline_metrics = compute_metrics(g, &baseline, &capacities);

    // T1 edge IDs
    let t1_routes = ["I5", "I10", "I35", "I40", "I75", "I80", "I90", "I95"];
    let candidate_edges: Vec<EdgeIndex> = if config.t1_only {
        g.graph
            .edge_indices()
            .filter(|&ei| t1_routes.contains(&g.graph[ei].route_id.as_str()))
            .collect()
    } else {
        g.graph.edge_indices().collect()
    };

    if candidate_edges.is_empty() {
        return ChaosResult {
            iterations: 0,
            mean_freight_cost_delta_m: 0.0,
            p95_freight_cost_delta_m: 0.0,
            max_freight_cost_delta_m: 0.0,
            worst_case_corridors: vec![],
            mean_network_pti: 1.0,
            saturation_fraction: 0.0,
        };
    }

    let mut rng = StdRng::seed_from_u64(config.seed);
    let mut cost_deltas: Vec<(f64, String)> = Vec::new();
    let mut pti_sum = 0.0f64;
    let mut saturation_count = 0usize;

    for _ in 0..config.iterations {
        let mut modified_caps = capacities.clone();
        let mut closed_routes: Vec<String> = Vec::new();

        // Random closures
        for &ei in &candidate_edges {
            if rng.gen::<f64>() < config.closure_probability {
                modified_caps.insert(ei, 0.0);
                closed_routes.push(g.graph[ei].route_id.clone());
            }
        }

        if modified_caps == capacities {
            // No closures — skip
            continue;
        }

        // Re-run equilibrium with modified capacities
        let scenario_flow = wardrop_equilibrium(
            g,
            demand,
            &modified_caps,
            &bpr,
            config.fw_max_iter,
            config.fw_tolerance,
        );
        let scenario_metrics = compute_metrics(g, &scenario_flow, &modified_caps);
        let cost_delta =
            scenario_metrics.freight_cost_per_hour_m - baseline_metrics.freight_cost_per_hour_m;

        closed_routes.dedup();
        let route_label = closed_routes.join("+");
        cost_deltas.push((cost_delta, route_label));
        pti_sum += scenario_metrics.mean_pti;

        if scenario_metrics.losf_edges > 0 {
            saturation_count += 1;
        }
    }

    if cost_deltas.is_empty() {
        return ChaosResult {
            iterations: config.iterations,
            mean_freight_cost_delta_m: 0.0,
            p95_freight_cost_delta_m: 0.0,
            max_freight_cost_delta_m: 0.0,
            worst_case_corridors: vec![],
            mean_network_pti: 1.0,
            saturation_fraction: 0.0,
        };
    }

    cost_deltas.sort_by(|a, b| a.0.total_cmp(&b.0));
    let n = cost_deltas.len() as f64;
    let mean = cost_deltas.iter().map(|(c, _)| c).sum::<f64>() / n;
    let p95_idx = ((n * 0.95) as usize).min(cost_deltas.len() - 1);
    let max = cost_deltas.last().map(|(c, _)| *c).unwrap_or(0.0);

    // Worst 5% corridors
    let worst_5pct = &cost_deltas[p95_idx..];
    let mut worst_routes: HashMap<String, usize> = HashMap::new();
    for (_, route) in worst_5pct {
        *worst_routes.entry(route.clone()).or_insert(0) += 1;
    }
    let mut worst_vec: Vec<(String, usize)> = worst_routes.into_iter().collect();
    worst_vec.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
    let worst_case_corridors = worst_vec.into_iter().map(|(r, _)| r).take(5).collect();

    ChaosResult {
        iterations: cost_deltas.len(),
        mean_freight_cost_delta_m: mean,
        p95_freight_cost_delta_m: cost_deltas[p95_idx].0,
        max_freight_cost_delta_m: max,
        worst_case_corridors,
        mean_network_pti: pti_sum / n,
        saturation_fraction: saturation_count as f64 / n,
    }
}
