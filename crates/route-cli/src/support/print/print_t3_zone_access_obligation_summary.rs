//! Helper `print_t3_zone_access_obligation_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t3_zone_access_obligation_summary(output: &Path, rows: &[T3ZoneAccessObligationRow]) {
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
        *by_class.entry(row.obligation_class.as_str()).or_default() += 1;
    }

    println!(
        "  wrote {} T3 zone access obligation rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
    for (class, count) in by_class {
        println!("  {class}: {count}");
    }
}

