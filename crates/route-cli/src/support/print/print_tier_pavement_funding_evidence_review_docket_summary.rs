//! Helper `print_tier_pavement_funding_evidence_review_docket_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_review_docket_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceReviewDocketRow],
) {
    println!(
        "  wrote {} pavement funding evidence review rows to {}",
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

