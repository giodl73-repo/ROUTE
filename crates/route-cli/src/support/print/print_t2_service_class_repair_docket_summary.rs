//! Helper `print_t2_service_class_repair_docket_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_service_class_repair_docket_summary(
    output: &Path,
    rows: &[T2ServiceClassRepairDocketRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.service_repair_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 service-class repair docket rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in counts {
        println!("  {class}: {count}");
    }
}

