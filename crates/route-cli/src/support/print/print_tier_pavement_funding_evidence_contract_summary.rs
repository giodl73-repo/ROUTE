//! Helper `print_tier_pavement_funding_evidence_contract_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_funding_evidence_contract_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceContractRow],
) {
    println!(
        "  wrote {} pavement funding evidence contract rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} min ${:.2}M",
            row.state, row.route, row.accepted_evidence_status, row.minimum_commitment_amount_m
        );
    }
}
