//! Helper `print_t2_stitched_member_proof_artifact_attachment_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_stitched_member_proof_artifact_attachment_summary(
    output: &Path,
    rows: &[T2StitchedMemberProofArtifactAttachmentRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.attachment_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member proof artifact-attachment rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

