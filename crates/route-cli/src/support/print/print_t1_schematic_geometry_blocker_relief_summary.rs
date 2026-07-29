//! Helper `print_t1_schematic_geometry_blocker_relief_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_schematic_geometry_blocker_relief_summary(
    output: &Path,
    rows: &[T1SchematicGeometryBlockerReliefRow],
) {
    let before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T1 schematic-geometry blocker relief rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers before: {before}");
    println!("  claim blockers after: {after}");
}

