//! Helper `print_t2_local_zone_overlay_handoff_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_local_zone_overlay_handoff_summary(
    output: &Path,
    rows: &[T2LocalZoneOverlayHandoffRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.handoff_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 local-zone overlay handoff rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}
