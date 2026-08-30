//! Helper `print_tier_pavement_funding_evidence_source_access_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_source_access_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceSourceAccessRow],
) {
    println!(
        "  wrote {} pavement funding evidence source-access rows to {}",
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
