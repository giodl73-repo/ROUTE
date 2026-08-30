//! Helper `print_tier_pavement_funding_evidence_accepted_source_access_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_accepted_source_access_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedSourceAccessRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted source-access rows to {}",
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
