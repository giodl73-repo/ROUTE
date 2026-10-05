//! Helper `print_t1_stop_selector_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_stop_selector_summary(output: &Path, rows: &[T1StopSelectorRow]) {
    let route_count = rows
        .iter()
        .map(|row| row.route.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let boundary_count = rows.iter().filter(|row| row.boundary_after).count();
    println!(
        "  wrote {} stop rows across {} T1 routes to {}",
        rows.len(),
        route_count,
        output.display()
    );
    println!("  METIS split boundaries: {boundary_count}");
}
