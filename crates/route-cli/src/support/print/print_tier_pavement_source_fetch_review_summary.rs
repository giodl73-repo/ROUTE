//! Helper `print_tier_pavement_source_fetch_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_source_fetch_review_summary(
    output: &Path,
    rows: &[TierPavementSourceFetchReviewRow],
) {
    println!(
        "  wrote {} pavement source-fetch review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} unresolved {}",
            row.task_id, row.state, row.join_review_status, row.postfetch_unresolved_member_count
        );
    }
}
