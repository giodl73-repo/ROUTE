//! Helper `print_tier_pavement_funding_evidence_intake_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_intake_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceIntakeRow],
) {
    println!(
        "  wrote {} pavement funding evidence intake rows to {}",
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

