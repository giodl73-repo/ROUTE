/// Named scenario execution.
///
/// A scenario is a named, reproducible simulation run that answers a specific
/// "what if" question about the national highway network. Scenarios are defined
/// in TOML files in crates/route-sim/src/scenarios/.
///
/// Each scenario specifies:
///   - An incident (closure, partial reduction, weather event)
///   - An optional intervention (managed lanes, diamond, etc.)
///   - The measurement: what metrics to report
///
/// The output is a ScenarioResult: baseline vs. incident vs. intervention comparison.
use crate::assignment::{edge_capacity_vph, wardrop_equilibrium, BprParams};
use crate::demand::DemandMatrix;
use crate::incident::{apply_incident, IncidentSpec};
use crate::metrics::{compute_metrics, corridor_pti, freight_cost_delta, SimMetrics};
use petgraph::graph::EdgeIndex;
use route_network::HighwayGraph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A named simulation scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub incident: IncidentSpec,
    /// Optional intervention: additional edges added OR capacity increased
    pub intervention: Option<Intervention>,
    /// Corridors to report PTI for
    pub report_corridors: Vec<String>,
    pub fw_max_iter: usize,
    pub fw_tolerance: f64,
}

/// An I2.0 intervention to test against the incident.
/// Uses `kind` as the TOML discriminant (not `type` — toml-serde limitation).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Intervention {
    /// Add managed freight lanes (capacity increase on existing edges)
    ManagedLanes {
        corridor: String,
        added_lanes_per_direction: u8,
    },
    /// Diamond interchange: add connector edges at T1/T1 intersection
    Diamond {
        intersection_name: String,
        connector_capacity_vph: f64,
    },
    /// Remove incident constraint (theoretical baseline — no incident ever occurs)
    NoIncident,
}

/// Full results from running a scenario.
#[derive(Debug)]
pub struct ScenarioResult {
    pub scenario_name: String,
    pub baseline: RunResult,
    pub incident: RunResult,
    /// Intervention result if an intervention was specified
    pub intervention: Option<RunResult>,
}

#[derive(Debug)]
pub struct RunResult {
    pub label: String,
    pub metrics: SimMetrics,
    /// PTI per reported corridor
    pub corridor_ptis: HashMap<String, f64>,
    /// Freight cost delta vs free-flow ($M/peak-hour)
    pub freight_cost_delta_m: f64,
    /// T90 recovery time estimate (hours to reach 90% baseline throughput after incident)
    pub t90_hours: Option<f64>,
    pub fw_iterations: usize,
    pub fw_gap: f64,
}

/// Lightweight validation for scenario definitions before execution.
///
/// These warnings intentionally do not reject a scenario: early Milepost 4
/// fixtures are useful as named shells, but they must not be mistaken for
/// fully bound pressure tests until they name concrete graph edge IDs.
pub fn scenario_validation_warnings(scenario: &Scenario) -> Vec<String> {
    let mut warnings = Vec::new();

    if scenario.incident.affected_edges.is_empty() {
        warnings.push("incident has no affected_edges; no graph capacity will change".to_string());
    }
    if scenario.report_corridors.is_empty() {
        warnings.push("report_corridors is empty; no corridor PTI will be reported".to_string());
    }
    if scenario.fw_max_iter == 0 {
        warnings.push("fw_max_iter is zero; assignment will not iterate".to_string());
    }
    if !scenario.fw_tolerance.is_finite() || scenario.fw_tolerance <= 0.0 {
        warnings.push("fw_tolerance must be finite and positive".to_string());
    }
    if !scenario.incident.duration_hours.is_finite() || scenario.incident.duration_hours <= 0.0 {
        warnings.push("incident duration_hours must be finite and positive".to_string());
    }
    if !scenario.incident.annual_occurrences.is_finite()
        || scenario.incident.annual_occurrences < 0.0
    {
        warnings.push("incident annual_occurrences must be finite and non-negative".to_string());
    }

    if let Some(intervention) = &scenario.intervention {
        match intervention {
            Intervention::ManagedLanes {
                added_lanes_per_direction,
                ..
            } if *added_lanes_per_direction == 0 => {
                warnings.push(
                    "managed-lanes intervention adds zero lanes; scenario is a named placeholder"
                        .to_string(),
                );
            }
            Intervention::Diamond {
                connector_capacity_vph,
                ..
            } if !connector_capacity_vph.is_finite() || *connector_capacity_vph <= 0.0 => {
                warnings
                    .push("diamond connector_capacity_vph must be finite and positive".to_string());
            }
            _ => {}
        }
    }

    warnings
}

/// Run a named scenario and return the full comparison.
pub fn run_scenario(
    g: &HighwayGraph,
    demand: &DemandMatrix,
    scenario: &Scenario,
) -> ScenarioResult {
    let bpr = BprParams::default();
    let base_capacities: HashMap<EdgeIndex, f64> = g
        .graph
        .edge_indices()
        .map(|ei| (ei, edge_capacity_vph(g, ei)))
        .collect();

    let edge_id_map: HashMap<u64, EdgeIndex> = g
        .graph
        .edge_indices()
        .map(|ei| (g.graph[ei].id, ei))
        .collect();

    // --- Baseline ---
    let baseline_flow = wardrop_equilibrium(
        g,
        demand,
        &base_capacities,
        &bpr,
        scenario.fw_max_iter,
        scenario.fw_tolerance,
    );
    let baseline_result = build_run_result(
        "baseline",
        g,
        &baseline_flow,
        &base_capacities,
        &scenario.report_corridors,
    );

    // --- Incident ---
    let (incident_caps, _snapshot) =
        apply_incident(&base_capacities, &scenario.incident, &edge_id_map);
    let incident_flow = wardrop_equilibrium(
        g,
        demand,
        &incident_caps,
        &bpr,
        scenario.fw_max_iter,
        scenario.fw_tolerance,
    );
    let mut incident_result = build_run_result(
        "incident",
        g,
        &incident_flow,
        &incident_caps,
        &scenario.report_corridors,
    );

    // Estimate T90: simple heuristic based on throughput drop
    let baseline_throughput = baseline_result.metrics.total_throughput_vph;
    let incident_throughput = incident_result.metrics.total_throughput_vph;
    let throughput_fraction = if baseline_throughput > 0.0 {
        incident_throughput / baseline_throughput
    } else {
        1.0
    };
    // T90 = incident duration × (1 - throughput_fraction) × recovery_factor
    // Simplified: if 50% throughput during 48h closure, T90 ≈ 12h after reopening
    incident_result.t90_hours =
        Some(scenario.incident.duration_hours * (1.0 - throughput_fraction) * 0.5);

    // --- Intervention (if any) ---
    let intervention_result = scenario.intervention.as_ref().map(|intervention| {
        let mut intervention_caps = incident_caps.clone();
        match intervention {
            Intervention::ManagedLanes {
                corridor,
                added_lanes_per_direction,
            } => {
                for &ei in g.route_edges(corridor) {
                    if let Some(c) = intervention_caps.get_mut(&ei) {
                        *c += *added_lanes_per_direction as f64 * 1_900.0;
                    }
                }
            }
            Intervention::Diamond {
                connector_capacity_vph,
                ..
            } => {
                // Diamond adds capacity at the intersection — modeled as capacity
                // increase on incident edges (simplified; real model would add new nodes/edges)
                for &ei in &scenario.incident.affected_edges {
                    if let Some(&idx) = edge_id_map.get(&ei) {
                        if let Some(c) = intervention_caps.get_mut(&idx) {
                            *c += connector_capacity_vph;
                        }
                    }
                }
            }
            Intervention::NoIncident => {
                intervention_caps = base_capacities.clone();
            }
        }

        let int_flow = wardrop_equilibrium(
            g,
            demand,
            &intervention_caps,
            &bpr,
            scenario.fw_max_iter,
            scenario.fw_tolerance,
        );
        build_run_result(
            "intervention",
            g,
            &int_flow,
            &intervention_caps,
            &scenario.report_corridors,
        )
    });

    ScenarioResult {
        scenario_name: scenario.name.clone(),
        baseline: baseline_result,
        incident: incident_result,
        intervention: intervention_result,
    }
}

#[cfg(test)]
mod tests {
    use super::{scenario_validation_warnings, Scenario};
    use crate::incident::{IncidentSpec, IncidentType};

    #[test]
    fn embedded_scenarios_parse() {
        for name in crate::scenarios::available_scenarios() {
            let toml = crate::scenarios::load_scenario(name).expect("scenario exists");
            let scenario: Scenario = toml::from_str(toml).expect("scenario parses");
            assert_eq!(scenario.name, *name);
        }
    }

    #[test]
    fn scenario_validation_flags_unbound_incident_edges() {
        let scenario = Scenario {
            name: "unbound".to_string(),
            description: "test".to_string(),
            incident: IncidentSpec {
                name: "unbound incident".to_string(),
                affected_edges: Vec::new(),
                incident_type: IncidentType::Closure,
                duration_hours: 1.0,
                annual_occurrences: 1.0,
            },
            intervention: None,
            report_corridors: vec!["I80".to_string()],
            fw_max_iter: 1,
            fw_tolerance: 0.01,
        };
        let warnings = scenario_validation_warnings(&scenario);

        assert!(warnings.iter().any(|w| w.contains("no affected_edges")));
    }
}

fn build_run_result(
    label: &str,
    g: &HighwayGraph,
    flow: &crate::assignment::FlowState,
    capacities: &HashMap<EdgeIndex, f64>,
    report_corridors: &[String],
) -> RunResult {
    let metrics = compute_metrics(g, flow, capacities);
    let corridor_ptis: HashMap<String, f64> = report_corridors
        .iter()
        .map(|c| (c.clone(), corridor_pti(g, c, flow, capacities)))
        .collect();

    // Freight cost delta vs zero-flow baseline
    let zero = crate::assignment::FlowState::empty();
    let freight_cost_delta_m = freight_cost_delta(g, &zero, flow, capacities);

    RunResult {
        label: label.to_string(),
        metrics,
        corridor_ptis,
        freight_cost_delta_m,
        t90_hours: None,
        fw_iterations: flow.iterations,
        fw_gap: flow.relative_gap,
    }
}
