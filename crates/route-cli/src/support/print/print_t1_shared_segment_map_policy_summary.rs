//! Helper `print_t1_shared_segment_map_policy_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_shared_segment_map_policy_summary(
    output: &Path,
    rows: &[T1SharedSegmentMapPolicyRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T1 shared-segment map policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}
