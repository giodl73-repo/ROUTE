//! Helper `print_tier_pavement_unmatched_join_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_unmatched_join_review_summary(
    output: &Path,
    rows: &[TierPavementUnmatchedJoinReviewRow],
) {
    println!(
        "  wrote {} pavement unmatched join review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} source-needed {} repair {} hpms-route-records {}",
            row.state,
            row.join_review_status,
            row.source_needed_member_count,
            row.repair_required_member_count,
            row.hpms_records_for_source_needed_routes
        );
    }
}

