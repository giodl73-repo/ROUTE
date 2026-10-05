//! Helper `print_t3_zone_map_diagnostic_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t3_zone_map_diagnostic_summary(output: &Path, rows: &[T3ZoneMapDiagnosticRow]) {
    let mut by_readiness = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_readiness.entry(row.map_readiness.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3 zone map diagnostic rows to {}",
        rows.len(),
        output.display()
    );
    for (readiness, count) in by_readiness {
        println!("  {readiness}: {count}");
    }
}
