//! Helper `print_tier_pavement_funding_evidence_accepted_metadata_source_capture_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_source_capture_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata source-capture rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.source_capture_status, row.captured_artifact
        );
    }
}

