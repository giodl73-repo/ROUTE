//! Helper `print_tier_pavement_funding_evidence_accepted_metadata_source_access_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_source_access_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata source-access rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.access_mode, row.evidence_artifact
        );
    }
}
