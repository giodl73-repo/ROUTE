//! Helper `print_significant_moments`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_significant_moments(rows: &[SignificantMomentRow], blockers: bool, details: bool) {
    let failures = significant_moment_gate_failures(rows);
    let failure_ids = failures
        .iter()
        .filter_map(|failure| failure.split_whitespace().next())
        .collect::<std::collections::HashSet<_>>();
    let filtered = if blockers {
        rows.iter()
            .filter(|row| {
                failure_ids.contains(row.moment_id.as_str()) || row.next_thread.trim().is_empty()
            })
            .collect::<Vec<_>>()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    let mut by_kind: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for row in rows {
        *by_kind.entry(row.kind.clone()).or_insert(0) += 1;
    }

    println!("route significant-moments");
    println!("  moments: {} shown / {} total", filtered.len(), rows.len());
    println!("  kind: {}", format_count_map(&by_kind));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<34} {:<10} {:<26} {:<24} {}",
        "Moment", "Date", "Flair", "Kind", "Summary"
    );
    println!("{}", "-".repeat(128));
    for row in filtered {
        println!(
            "{:<34} {:<10} {:<26} {:<24} {}",
            row.moment_id,
            row.date,
            truncate_for_table(&row.flair, 26),
            truncate_for_table(&row.kind, 24),
            row.summary
        );
        if details {
            println!("  why: {}", row.why_it_mattered);
            println!("  artifacts: {}", row.primary_artifacts);
            println!("  commit: {}", row.commit);
            println!("  next: {}", row.next_thread);
        }
    }
}

