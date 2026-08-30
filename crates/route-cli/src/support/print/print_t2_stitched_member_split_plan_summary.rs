//! Helper `print_t2_stitched_member_split_plan_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_stitched_member_split_plan_summary(
    output: &Path,
    rows: &[T2StitchedMemberSplitPlanRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.route.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member split plan rows to {}",
        rows.len(),
        output.display()
    );
    for (route, count) in counts {
        println!("  {route}: {count}");
    }
}
