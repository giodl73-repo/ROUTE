//! Helper `print_t1_snapshot_script`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_snapshot_script(rows: &[T1SnapshotPlanRow], priority: Option<&str>) {
    let filtered = filtered_t1_snapshot_rows(rows, priority);

    println!("route t1-snapshot-plan --script");
    println!("  feeds: {} shown / {} total", filtered.len(), rows.len());
    println!();
    for row in filtered {
        println!(
            "# {} {} ({})",
            row.site_id, row.intersection, row.source_name
        );
        println!("{}", row.fetch_command);
        println!("{}", row.import_command);
        println!("{}", row.accumulate_command);
        println!();
    }
}
