//! Helper `print_tier_pavement_downgrade_exclusion_decision_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_downgrade_exclusion_decision_summary(
    output: &Path,
    rows: &[TierPavementDowngradeExclusionDecisionRow],
) {
    println!(
        "  wrote {} pavement downgrade/exclusion decision rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.downgrade_decision, row.exclusion_decision
        );
    }
}

