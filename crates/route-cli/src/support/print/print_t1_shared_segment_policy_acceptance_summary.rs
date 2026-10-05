//! Helper `print_t1_shared_segment_policy_acceptance_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_shared_segment_policy_acceptance_summary(
    output: &Path,
    rows: &[T1SharedSegmentPolicyAcceptanceRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T1 shared-segment policy acceptance rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}
