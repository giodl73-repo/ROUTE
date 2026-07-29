//! Helper `print_tier_pavement_repair_debt_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_repair_debt_review_summary(
    output: &Path,
    rows: &[TierPavementRepairDebtReviewRow],
) {
    println!(
        "  wrote {} pavement repair debt review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} members {} repair ${:.2}M",
            row.state,
            row.route,
            row.repair_debt_status,
            row.blocked_member_count,
            row.estimated_repair_cost_m
        );
    }
}

