//! Helper `print_throughput_proof_matrix`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_throughput_proof_matrix(
    rows: &[ThroughputProofRow],
    blockers: bool,
    details: bool,
) {
    let failures = throughput_proof_gate_failures(rows);
    let filtered = if blockers {
        failures.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };
    let mut by_binding: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_binding.entry(row.binding_type.clone()).or_insert(0) += 1;
    }

    println!("route throughput-proof");
    println!("  rows: {} shown / {} total", filtered.len(), rows.len());
    println!("  binding: {}", format_count_map(&by_binding));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<18} {:<26} {:<20} {:<12} {}",
        "Proof", "Name", "Binding", "Status", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<26} {:<20} {:<12} {}",
            row.proof_id,
            truncate_for_table(&row.proof_name, 26),
            row.binding_type,
            row.current_status,
            row.blocking_gap
        );
        if details {
            println!("  stressor: {}", row.stressor);
            println!("  metric: {}", row.primary_metric);
            println!("  artifact: {}", row.existing_artifact);
            println!("  next: {}", row.next_evidence_step);
        }
    }
}
