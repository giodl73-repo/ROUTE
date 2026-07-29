//! Helper `print_standards_proof`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_standards_proof(
    rows: &[StandardsProofRow],
    tier: Option<&str>,
    family: Option<&str>,
    details: bool,
) {
    let filtered: Vec<&StandardsProofRow> = rows
        .iter()
        .filter(|row| {
            tier.map(|t| row.tier.eq_ignore_ascii_case(t))
                .unwrap_or(true)
                && family
                    .map(|f| row.standard_family.eq_ignore_ascii_case(f))
                    .unwrap_or(true)
        })
        .collect();

    println!("route standards-proof");
    println!(
        "  standards: {} shown / {} total",
        filtered.len(),
        rows.len()
    );

    let mut by_level: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for row in &filtered {
        *by_level.entry(row.evidence_level.clone()).or_insert(0) += 1;
    }
    if !by_level.is_empty() {
        let summary = by_level
            .iter()
            .map(|(level, count)| format!("{level}: {count}"))
            .collect::<Vec<_>>()
            .join(", ");
        println!("  evidence: {summary}");
    }
    println!();

    println!(
        "{:<24} {:<4} {:<12} {:<11} {}",
        "Standard", "Tier", "Family", "Evidence", "Blocking gap"
    );
    println!("{}", "-".repeat(110));
    for row in filtered {
        println!(
            "{:<24} {:<4} {:<12} {:<11} {}",
            row.standard_id, row.tier, row.standard_family, row.evidence_level, row.blocking_gap
        );
        if details {
            println!("  standard: {}", row.standard);
            println!("  outcome: {}", row.outcome);
            println!("  mechanism: {}", row.mechanism);
            println!("  stressor: {}", row.primary_stressor);
            println!("  gate: {}", row.acceptance_gate);
            println!("  artifact: {}", row.current_artifact);
            println!("  next: {}", row.next_command_or_test);
            println!("  owner: {}", row.owner_track);
        }
    }
}

