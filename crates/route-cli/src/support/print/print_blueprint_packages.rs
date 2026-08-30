//! Helper `print_blueprint_packages`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_blueprint_packages(
    rows: &[BlueprintPackageRow],
    blockers: bool,
    details: bool,
) {
    let failures = blueprint_gate_failures(rows);
    let filtered = if blockers {
        rows.iter()
            .filter(|row| {
                !row.blocking_gap.trim().is_empty()
                    || failures
                        .iter()
                        .any(|failure| failure.starts_with(row.package_id.as_str()))
            })
            .collect::<Vec<_>>()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    let mut by_phase: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    let mut by_class: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for row in rows {
        *by_phase.entry(row.phase.clone()).or_insert(0) += 1;
        *by_class.entry(row.stakeholder_class.clone()).or_insert(0) += 1;
    }

    println!("route blueprint");
    println!(
        "  packages: {} shown / {} total",
        filtered.len(),
        rows.len()
    );
    println!("  phases: {}", format_count_map(&by_phase));
    println!("  stakeholder classes: {}", format_count_map(&by_class));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<14} {:<8} {:<30} {:<24} {:<10} {}",
        "Package", "Phase", "Feature", "Class", "Evidence", "Action"
    );
    println!("{}", "-".repeat(126));
    for row in filtered {
        println!(
            "{:<14} {:<8} {:<30} {:<24} {:<10} {}",
            row.package_id,
            row.phase,
            truncate_for_table(&row.feature_package, 30),
            truncate_for_table(&row.stakeholder_class, 24),
            row.evidence_level,
            row.blueprint_action
        );
        if details {
            println!("  standards: {}", row.standards);
            println!("  cost_range: {}", row.cost_range);
            println!("  value_case: {}", row.value_case);
            println!("  source_label: {}", row.source_label);
            println!("  pressure_artifact: {}", row.pressure_artifact);
            println!("  forum_constraint: {}", row.forum_constraint);
            println!("  mitigation_companion: {}", row.mitigation_companion);
            println!("  row_complexity: {}", row.row_complexity);
            println!("  maintenance_burden: {}", row.maintenance_burden);
            println!(
                "  community_exposure_check: {}",
                row.community_exposure_check
            );
            println!("  rural_access_exception: {}", row.rural_access_exception);
            println!("  gap: {}", row.blocking_gap);
            println!("  next: {}", row.next_evidence_step);
        }
    }
}
