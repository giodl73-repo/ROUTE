//! Helper `print_tier_pavement_funding_commitment_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_commitment_review_summary(
    output: &Path,
    rows: &[TierPavementFundingCommitmentReviewRow],
) {
    println!(
        "  wrote {} pavement funding commitment review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.funding_commitment_status, row.relief_eligibility
        );
    }
}

