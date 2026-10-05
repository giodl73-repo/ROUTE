//! Helper `print_endpoint_exceptions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_endpoint_exceptions(
    rows: &[&EndpointExceptionRow],
    blockers: bool,
    details: bool,
) {
    let visible = rows
        .iter()
        .copied()
        .filter(|row| {
            !blockers
                || !endpoint_exception_has_contract(row)
                || !endpoint_exception_is_terminal_worthy(row)
        })
        .collect::<Vec<_>>();
    let mut by_tier = std::collections::BTreeMap::new();
    let mut by_status = std::collections::BTreeMap::new();
    for row in &visible {
        *by_tier
            .entry(row.requested_tier.trim().to_ascii_uppercase())
            .or_insert(0usize) += 1;
        *by_status
            .entry(row.evidence_level.trim().to_ascii_lowercase())
            .or_insert(0usize) += 1;
    }

    println!("  exceptions: {}", visible.len());
    println!("  tier mix: {}", format_count_map(&by_tier));
    println!("  evidence mix: {}", format_count_map(&by_status));
    println!();
    println!(
        "{:<8} {:<5} {:<26} {:<24} {:<22} Worthy",
        "Route", "Tier", "Endpoint", "Role", "Exception"
    );
    println!("{}", "-".repeat(104));
    for row in visible {
        println!(
            "{:<8} {:<5} {:<26} {:<24} {:<22} {}",
            normalise_designation(&row.route),
            row.requested_tier,
            truncate_for_table(&row.endpoint_name, 26),
            truncate_for_table(&row.endpoint_role, 24),
            truncate_for_table(&row.exception_type, 22),
            if endpoint_exception_is_terminal_worthy(row) {
                "yes"
            } else {
                "no"
            }
        );
        if details {
            println!("  evidence: {}", row.evidence_level);
            println!("  artifact: {}", row.artifact);
            println!("  next: {}", row.next_step);
        }
    }
}
