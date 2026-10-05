//! Helper `print_national_segment_registry_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_national_segment_registry_summary(
    output: &Path,
    rows: &[NationalSegmentRegistryRow],
) {
    let mut by_action = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_action.entry(row.registry_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} national segment registry rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in by_action {
        println!("  {action}: {count}");
    }
}
