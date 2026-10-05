//! Helper `print_chaos_result`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_chaos_result(result: &route_sim::ChaosResult) {
    println!("\n=== Chaos Results ({} iterations) ===", result.iterations);
    println!(
        "  Mean freight cost delta: +${:.2}M/peak-hr",
        result.mean_freight_cost_delta_m
    );
    println!(
        "  P95 freight cost delta:  +${:.2}M/peak-hr",
        result.p95_freight_cost_delta_m
    );
    println!(
        "  Max freight cost delta:  +${:.2}M/peak-hr",
        result.max_freight_cost_delta_m
    );
    println!("  Mean network PTI:        {:.2}", result.mean_network_pti);
    println!(
        "  Saturation fraction:     {:.1}%",
        result.saturation_fraction * 100.0
    );
    if !result.worst_case_corridors.is_empty() {
        println!(
            "  Worst-case corridors:    {}",
            result.worst_case_corridors.join(", ")
        );
    }
}
