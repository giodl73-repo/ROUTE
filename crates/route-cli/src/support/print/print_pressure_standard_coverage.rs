//! Helper `print_pressure_standard_coverage`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_pressure_standard_coverage(
    standards: &[StandardsProofRow],
    scenarios: &[PressureScenarioRow],
) {
    let scenario_refs = pressure_standard_scenario_refs(scenarios);
    let focus = pressure_standard_coverage_focus(standards);
    let failures = pressure_standard_coverage_failures(standards, scenarios);
    let unknown = pressure_scenario_unknown_standard_refs(standards, scenarios);

    println!();
    println!("Pressure standard coverage");
    println!(
        "  high-stakes T1 throughput/resilience standards: {}",
        focus.len()
    );
    println!(
        "  scenario-covered: {}",
        focus
            .iter()
            .filter(|row| scenario_refs.contains_key(row.standard_id.as_str()))
            .count()
    );
    println!("  missing hooks: {}", failures.len());
    println!("  unknown refs: {}", unknown.len());
    println!();
    println!(
        "{:<22} {:<12} {:<8} {:<22} {}",
        "Standard", "Family", "Evidence", "Scenario", "Stressor"
    );
    println!("{}", "-".repeat(112));
    for row in focus {
        let scenario = scenario_refs
            .get(row.standard_id.as_str())
            .map(|ids| ids.join("; "))
            .unwrap_or_else(|| "missing".to_string());
        println!(
            "{:<22} {:<12} {:<8} {:<22} {}",
            row.standard_id,
            row.standard_family,
            row.evidence_level,
            truncate_for_table(&scenario, 22),
            row.primary_stressor
        );
    }
    if !unknown.is_empty() {
        println!();
        println!("  unknown scenario standard refs: {}", unknown.join(", "));
    }
}
