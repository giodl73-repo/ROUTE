//! Helper `print_map_publication_readiness_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_map_publication_readiness_summary(
    output: &Path,
    rows: &[MapPublicationReadinessRow],
    details: bool,
) {
    println!(
        "  wrote {} readiness rows to {}",
        rows.len(),
        output.display()
    );
    if let Some(row) = rows.first() {
        println!("  maps: {} ({})", row.map_count, row.map_types);
        println!("  render gate: {}", row.render_gate_status);
        println!("  publication blockers: {}", row.publication_blocker_count);
        println!("  held claims: {}", row.held_claims);
        println!("  status: {}", row.validation_status);
        if details {
            println!("  decision: {}", row.readiness_decision);
            println!(
                "  publication families: {}",
                row.publication_blocker_families
            );
        }
    }
}
