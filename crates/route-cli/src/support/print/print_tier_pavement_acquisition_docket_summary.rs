//! Helper `print_tier_pavement_acquisition_docket_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_acquisition_docket_summary(
    output: &Path,
    rows: &[TierPavementAcquisitionDocketRow],
    priority: Option<&str>,
    script: bool,
) {
    let filtered = rows
        .iter()
        .filter(|row| {
            priority
                .map(|priority| row.source_priority.eq_ignore_ascii_case(priority))
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();
    let mut by_priority = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_priority.entry(row.source_priority.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pavement acquisition docket rows to {}",
        rows.len(),
        output.display()
    );
    println!("  rows shown: {} / {}", filtered.len(), rows.len());
    for (priority, count) in by_priority {
        println!("  priority {priority}: {count}");
    }

    if script {
        println!();
        for row in filtered {
            println!("# {} {} {}", row.task_id, row.state, row.affected_routes);
            println!("{}", row.fetch_command);
            println!("{}", row.rebuild_command);
            println!("{}", row.verify_command);
            println!();
        }
    }
}

