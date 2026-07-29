//! Helper `print_tier_pavement_funding_evidence_accepted_metadata_intake_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_intake_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataIntakeRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata intake rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.intake_status, row.evidence_artifact
        );
    }
}

