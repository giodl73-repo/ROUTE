//! Helper `print_t2_overlay_optimizer_action_docket_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_overlay_optimizer_action_docket_summary(
    output: &Path,
    rows: &[T2OverlayOptimizerActionDocketRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.optimizer_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 overlay optimizer action rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}
