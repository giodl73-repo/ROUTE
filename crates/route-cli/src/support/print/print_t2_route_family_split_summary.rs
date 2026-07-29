//! Helper `print_t2_route_family_split_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_route_family_split_summary(output: &Path, rows: &[T2RouteFamilySplitRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.family_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 route-family split rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

