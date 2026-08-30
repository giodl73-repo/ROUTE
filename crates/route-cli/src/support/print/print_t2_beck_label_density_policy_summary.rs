//! Helper `print_t2_beck_label_density_policy_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_beck_label_density_policy_summary(
    output: &Path,
    rows: &[T2BeckLabelDensityPolicyRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck label-density policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}
