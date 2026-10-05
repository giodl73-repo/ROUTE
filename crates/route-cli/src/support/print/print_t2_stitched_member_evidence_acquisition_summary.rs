//! Helper `print_t2_stitched_member_evidence_acquisition_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_stitched_member_evidence_acquisition_summary(
    output: &Path,
    rows: &[T2StitchedMemberEvidenceAcquisitionRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.acquisition_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member evidence acquisition rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}
