//! Helper `print_forum_docket`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_forum_docket(rows: &[ForumDocketRow], blockers: bool, details: bool) {
    let failures = forum_docket_gate_failures(rows);
    let failure_ids = failures
        .iter()
        .filter_map(|failure| failure.split_whitespace().next())
        .collect::<std::collections::HashSet<_>>();
    let filtered = if blockers {
        rows.iter()
            .filter(|row| {
                row.status.eq_ignore_ascii_case("planned")
                    || row.status.eq_ignore_ascii_case("held")
                    || failure_ids.contains(row.review_id.as_str())
            })
            .collect::<Vec<_>>()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_status.entry(row.status.clone()).or_insert(0) += 1;
    }

    println!("route forum");
    println!("  reviews: {} shown / {} total", filtered.len(), rows.len());
    println!("  status: {}", format_count_map(&by_status));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<18} {:<12} {:<10} {:<30} {}",
        "Review", "Type", "Status", "Artifact", "Question"
    );
    println!("{}", "-".repeat(122));
    for row in filtered {
        println!(
            "{:<18} {:<12} {:<10} {:<30} {}",
            row.review_id,
            row.review_type,
            row.status,
            truncate_for_table(&row.artifact, 30),
            row.blocking_question
        );
        if details {
            println!("  roles: {}", row.roles);
            println!("  claim: {}", row.claim_target);
            println!("  next: {}", row.next_action);
            println!("  output: {}", row.output_artifact);
        }
    }
}

