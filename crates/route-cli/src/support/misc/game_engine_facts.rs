//! Helper `game_engine_facts`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn game_engine_facts(scenario_id: &str, manifest_path: &Path) -> Result<Option<game::EngineFacts>> {
    if scenario_id != game::DES_MOINES_SCENARIO_ID {
        return Ok(None);
    }

    let manifest = route_data::Manifest::load(manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;
    let demand = build_demand_from_graph(&graph);

    let toml_str = route_sim::scenarios::load_scenario("des-moines-interchange")
        .ok_or_else(|| anyhow::anyhow!("missing embedded des-moines-interchange scenario"))?;
    let scenario: route_sim::Scenario =
        toml::from_str(toml_str).context("parsing des-moines-interchange scenario")?;
    let result = route_sim::run_scenario(&graph, &demand, &scenario);
    let intersection = route_network::find_intersection(&graph, "I35xI80")
        .ok_or_else(|| anyhow::anyhow!("missing I35xI80 diamond anchor"))?;
    let diamond = route_network::analyze_diamond(&graph, intersection);

    Ok(Some(game::EngineFacts {
        baseline_throughput_vph: result
            .baseline
            .metrics
            .total_throughput_vph
            .round()
            .max(0.0) as u32,
        incident_throughput_vph: result
            .incident
            .metrics
            .total_throughput_vph
            .round()
            .max(0.0) as u32,
        intervention_throughput_vph: result
            .intervention
            .as_ref()
            .map(|run| run.metrics.total_throughput_vph.round().max(0.0) as u32)
            .unwrap_or(0),
        recovery_hours: result.incident.t90_hours.unwrap_or(0.0),
        diamond_k_current: diamond.k_current.min(u8::MAX as usize) as u8,
        connectors_needed: diamond.connectors_needed.min(u8::MAX as usize) as u8,
        evidence_level: "Heuristic live ROUTE summary",
    }))
}

