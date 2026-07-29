//! Helper `print_t2_endpoint_closure_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_endpoint_closure_summary(output: &Path, rows: &[T2EndpointClosureRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.endpoint_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 endpoint closure rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

