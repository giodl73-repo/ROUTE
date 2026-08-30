//! Helper `print_t1_schematic_geometry_claim_review_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_schematic_geometry_claim_review_summary(
    output: &Path,
    rows: &[T1SchematicGeometryClaimReviewRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T1 schematic-geometry claim review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}
