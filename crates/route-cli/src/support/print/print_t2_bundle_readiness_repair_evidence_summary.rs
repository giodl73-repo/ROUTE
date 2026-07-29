//! Helper `print_t2_bundle_readiness_repair_evidence_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_bundle_readiness_repair_evidence_summary(
    output: &Path,
    rows: &[T2BundleReadinessRepairEvidenceRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.evidence_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle readiness repair evidence rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

