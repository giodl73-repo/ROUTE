//! Helper `print_t2_national_bundle_readiness_audit_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_national_bundle_readiness_audit_summary(
    output: &Path,
    rows: &[T2NationalBundleReadinessAuditRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.bundle_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 national bundle readiness audit rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

