//! Helper `print_t1_evidence_windows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_evidence_windows(
    rows: &[T1EvidenceWindowRow],
    blockers: bool,
    details: bool,
) {
    let filtered: Vec<&T1EvidenceWindowRow> = rows
        .iter()
        .filter(|row| !blockers || !row.promotion_eligible)
        .collect();
    let mut by_mode: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for row in rows {
        *by_mode.entry(row.evidence_mode.clone()).or_insert(0) += 1;
    }
    let eligible = rows.iter().filter(|row| row.promotion_eligible).count();

    println!("route t1-evidence-windows");
    println!("  windows: {} shown / {} total", filtered.len(), rows.len());
    println!("  evidence modes: {}", format_count_map(&by_mode));
    println!("  promotion eligible: {}", eligible);
    println!();
    println!(
        "{:<18} {:<18} {:<28} {:<16} {:>6} {:<9} {}",
        "Window", "Site", "Source", "Mode", "Events", "Promote", "Next"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<18} {:<28} {:<16} {:>6} {:<9} {}",
            row.window_id,
            row.site_id,
            truncate_for_table(&row.source_name, 28),
            row.evidence_mode,
            row.event_count,
            if row.promotion_eligible { "yes" } else { "no" },
            row.next_step
        );
        if details {
            println!(
                "  capture: {} -> {}; observations: {} -> {}",
                row.capture_started_at,
                row.capture_ended_at,
                row.observation_start,
                row.observation_end
            );
            println!(
                "  artifacts: {}; {}",
                row.raw_artifact, row.normalized_artifact
            );
            println!("  freight rows: {}", row.freight_relevant_count);
            println!("  gap: {}", row.blocking_gap);
            println!("  review: {}", row.review_artifact);
        }
    }
}
