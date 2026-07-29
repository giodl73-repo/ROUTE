//! Helper `print_t2_service_diagnostic_queue_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_service_diagnostic_queue_summary(output: &Path, rows: &[T2ServiceDiagnosticQueueRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.diagnostic_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} service diagnostic queue rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

