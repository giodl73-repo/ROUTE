//! Helper `print_tier_pavement_funding_evidence_accepted_attachment_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_accepted_attachment_review_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedAttachmentReviewRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted-attachment review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.review_decision, row.accepted_evidence_status
        );
    }
}
