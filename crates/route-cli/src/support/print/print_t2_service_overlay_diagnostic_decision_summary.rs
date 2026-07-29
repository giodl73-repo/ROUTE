//! Helper `print_t2_service_overlay_diagnostic_decision_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_service_overlay_diagnostic_decision_summary(
    output: &Path,
    rows: &[T2ServiceOverlayDiagnosticDecisionRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.overlay_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 service overlay diagnostic decision rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

