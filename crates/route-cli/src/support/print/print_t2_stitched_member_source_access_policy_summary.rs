//! Helper `print_t2_stitched_member_source_access_policy_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_stitched_member_source_access_policy_summary(
    output: &Path,
    rows: &[T2StitchedMemberSourceAccessPolicyRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.access_mode.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member source access policy rows to {}",
        rows.len(),
        output.display()
    );
    for (mode, count) in counts {
        println!("  {mode}: {count}");
    }
}

