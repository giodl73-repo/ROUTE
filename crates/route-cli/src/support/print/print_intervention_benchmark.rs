//! Helper `print_intervention_benchmark` (support::print).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_intervention_benchmark(bench: &route_sim::InterventionBenchmark) {
    let baseline_p95 = bench.baseline.p95_hours;
    let ff = bench.baseline.free_flow_hours;

    println!("Corridor: {}", bench.corridor_name);
    println!(
        "Baseline: Solo/GP lanes  |  free-flow {:.1}h  |  p95 {:.1}h ({:.1} days)\n",
        ff,
        baseline_p95,
        baseline_p95 / 24.0
    );

    // Header
    println!(
        "{:<35} {:>8}  {:>8}  {:>9}  {:>8}  {:>12}  {}",
        "Intervention", "p50", "p95", "Δp95", "< 48h", "Capex", "48h SLA"
    );
    println!("{}", "─".repeat(105));

    // Sort by p95 ascending (best first), keeping baseline at top
    let mut results: Vec<&route_sim::InterventionResult> = bench.results.iter().collect();
    results.sort_by(|a, b| a.dist.p95_hours.total_cmp(&b.dist.p95_hours));

    for r in &results {
        let delta_str = if r.p95_delta_hours.abs() < 0.05 {
            "  —    ".to_string()
        } else {
            format!("{:>+7.1}h", r.p95_delta_hours)
        };
        let sla = if r.sla_achieved { "✓ YES" } else { "✗ no " };
        let marker = if r.sla_achieved { " ←" } else { "" };
        println!(
            "{:<35} {:>6.1}h  {:>6.1}h  {}  {:>6.1}%  {:>12}  {}{}",
            r.label,
            r.dist.p50_hours,
            r.dist.p95_hours,
            delta_str,
            r.pct_under_48h,
            r.capex,
            sla,
            marker,
        );
    }

    println!("\n{}", "─".repeat(105));

    // Summary: rank by marginal impact
    let mut ranked: Vec<&route_sim::InterventionResult> = bench
        .results
        .iter()
        .filter(|r| {
            !r.label.contains("stack") && !r.label.contains("+") && !r.label.contains("Baseline")
        })
        .collect();
    ranked.sort_by(|a, b| a.p95_delta_hours.total_cmp(&b.p95_delta_hours));

    println!("\nRanked single interventions by p95 improvement:");
    println!(
        "{:<35} {:>9}  {:>14}  {:>12}",
        "Intervention", "p95 gain", "Cost/hour-saved", "Capex"
    );
    println!("{}", "─".repeat(80));
    for r in &ranked {
        let gain = baseline_p95 - r.dist.p95_hours;
        if gain.abs() < 0.1 {
            continue;
        }
        // Rough cost-per-hour-saved: capex / (gain × annual trips estimate)
        let annual_trips = 8_000.0 * 365.0; // 8k trucks/day on NY-LA
        let total_hours_saved = gain * annual_trips;
        // Parse capex to a number for $/hr calculation
        let cost_per_hour = if r.capex.contains("$0") {
            0.0
        } else if r.capex.contains("40M") {
            40_000_000.0 / total_hours_saved
        } else if r.capex.contains("200M") {
            200_000_000.0 / total_hours_saved
        } else if r.capex.contains("800M") {
            800_000_000.0 / total_hours_saved
        } else if r.capex.contains("930M") {
            930_000_000.0 / total_hours_saved
        } else if r.capex.contains("$4B") {
            4_000_000_000.0 / total_hours_saved
        } else if r.capex.contains("121B") {
            121_000_000_000.0 / total_hours_saved
        } else {
            -1.0
        };
        let cost_str = if cost_per_hour <= 0.0 {
            "free/operational".to_string()
        } else {
            format!("${:.2}/hr saved", cost_per_hour)
        };
        println!(
            "{:<35} {:>+8.1}h  {:>14}  {:>12}",
            r.label, -gain, cost_str, r.capex
        );
    }

    // Insight summary
    println!("\n── Key findings ─────────────────────────────────────────────────────");
    let achieves_48 = bench
        .results
        .iter()
        .filter(|r| r.sla_achieved && !r.label.contains("Baseline"))
        .map(|r| r.label.as_str())
        .collect::<Vec<_>>();
    if achieves_48.is_empty() {
        println!("  No single or combination intervention achieves 48h SLA on this corridor.");
    } else {
        println!("  48h SLA achieved by:");
        for label in &achieves_48 {
            println!("    ✓ {}", label);
        }
    }
    let best_value = ranked.first();
    if let Some(r) = best_value {
        let gain = baseline_p95 - r.dist.p95_hours;
        println!(
            "  Highest single-intervention impact: {} (−{:.1}h p95)",
            r.label, gain
        );
    }
}

