//! Helper `print_pressure_scenarios`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_pressure_scenarios(
    rows: &[PressureScenarioRow],
    blockers: bool,
    details: bool,
) {
    let failures = pressure_scenario_gate_failures(rows);
    let filtered = if blockers {
        failures.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };
    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_status.entry(row.current_status.clone()).or_insert(0) += 1;
    }

    println!("route pressure-scenarios");
    println!(
        "  scenarios: {} shown / {} total",
        filtered.len(),
        rows.len()
    );
    println!("  status: {}", format_count_map(&by_status));
    println!("  L2 gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<18} {:<24} {:<14} {:<28} {}",
        "Scenario", "Name", "Status", "Adversity", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<24} {:<14} {:<28} {}",
            row.scenario_id,
            truncate_for_table(&row.scenario_name, 24),
            row.current_status,
            truncate_for_table(&row.adversity_class, 28),
            row.blocking_gap
        );
        if details {
            println!("  standards: {}", row.standards_tested);
            println!("  artifact: {}", row.existing_artifact);
            println!("  next: {}", row.next_evidence_step);
        }
    }
}
