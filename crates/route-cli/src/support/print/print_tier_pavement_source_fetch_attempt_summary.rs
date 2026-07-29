//! Helper `print_tier_pavement_source_fetch_attempt_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_source_fetch_attempt_summary(
    output: &Path,
    rows: &[TierPavementSourceFetchAttemptRow],
) {
    println!(
        "  wrote {} pavement source-fetch attempt rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} records {}",
            row.task_id, row.state, row.cache_record_count, row.fetch_result_status
        );
    }
}

