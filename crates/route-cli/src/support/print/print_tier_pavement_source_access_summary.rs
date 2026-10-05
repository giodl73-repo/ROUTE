//! Helper `print_tier_pavement_source_access_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_source_access_summary(
    output: &Path,
    rows: &[TierPavementSourceAccessRow],
    priority: &str,
) {
    println!(
        "  wrote {} priority-{priority} pavement source-access rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} -> {}",
            row.task_id, row.state, row.source_access_mode, row.mutation_mode
        );
    }
}
