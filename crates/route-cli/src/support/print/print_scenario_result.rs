//! Helper `print_scenario_result`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_scenario_result(result: &route_sim::ScenarioResult) {
    println!("\n=== {} ===", result.scenario_name);
    println!(
        "  Baseline:  throughput {:.0} vph  |  PTI {:.2}  |  freight cost ${:.2}M/hr",
        result.baseline.metrics.total_throughput_vph,
        result.baseline.metrics.mean_pti,
        result.baseline.metrics.freight_cost_per_hour_m
    );
    println!(
        "  Incident:  throughput {:.0} vph  |  PTI {:.2}  |  freight cost ${:.2}M/hr",
        result.incident.metrics.total_throughput_vph,
        result.incident.metrics.mean_pti,
        result.incident.metrics.freight_cost_per_hour_m
    );
    println!(
        "  Cost delta: +${:.2}M/hr  |  LOS-F edges: {}  |  T90: {:.1}h",
        result.incident.freight_cost_delta_m,
        result.incident.metrics.losf_edges,
        result.incident.t90_hours.unwrap_or(0.0)
    );

    if let Some(ref int_result) = result.intervention {
        println!(
            "  Intervention: throughput {:.0} vph  |  PTI {:.2}  |  cost ${:.2}M/hr",
            int_result.metrics.total_throughput_vph,
            int_result.metrics.mean_pti,
            int_result.metrics.freight_cost_per_hour_m
        );
        let improvement = result.incident.metrics.freight_cost_per_hour_m
            - int_result.metrics.freight_cost_per_hour_m;
        println!(
            "  Intervention saves: ${:.2}M/hr  PTI improvement: {:.2} → {:.2}",
            improvement, result.incident.metrics.mean_pti, int_result.metrics.mean_pti
        );
    }

    // Corridor PTIs
    if !result.incident.corridor_ptis.is_empty() {
        println!("\n  Corridor PTIs (incident):");
        let mut ptis: Vec<(&String, &f64)> = result.incident.corridor_ptis.iter().collect();
        ptis.sort_by(|a, b| b.1.total_cmp(a.1));
        for (corridor, pti) in ptis {
            let flag = if *pti > 1.3 { " ⚠" } else { "" };
            println!("    {}: {:.2}{}", corridor, pti, flag);
        }
    }
}

