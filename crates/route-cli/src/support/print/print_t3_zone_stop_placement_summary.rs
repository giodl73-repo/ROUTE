//! Helper `print_t3_zone_stop_placement_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t3_zone_stop_placement_summary(output: &Path, rows: &[T3ZoneStopPlacementRow]) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.placement_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3 zone stop placement rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}
