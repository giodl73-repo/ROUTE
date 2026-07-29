//! Helper `print_tier_pavement_funding_evidence_accepted_artifact_attachment_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_accepted_artifact_attachment_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedArtifactAttachmentRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted-artifact attachment rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.attachment_status, row.attached_artifact
        );
    }
}

