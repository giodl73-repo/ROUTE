//! Helper `print_tier_pavement_funding_evidence_acquisition_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_acquisition_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcquisitionRow],
) {
    println!(
        "  wrote {} pavement funding evidence acquisition rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.acquisition_status, row.accepted_evidence_status
        );
    }
}

