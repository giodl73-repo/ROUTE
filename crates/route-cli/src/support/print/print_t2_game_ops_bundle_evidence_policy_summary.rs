//! Helper `print_t2_game_ops_bundle_evidence_policy_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_game_ops_bundle_evidence_policy_summary(
    output: &Path,
    rows: &[T2GameOpsBundleEvidencePolicyRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 game/ops bundle evidence policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}
