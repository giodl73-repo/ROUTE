//! Helper `print_source_fetch_policy_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_source_fetch_policy_summary(output: &Path, rows: &[SourceFetchPolicyRow]) {
    let mut modes = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *modes.entry(row.mutation_mode.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} source fetch policy rows to {}",
        rows.len(),
        output.display()
    );
    for (mode, count) in modes {
        println!("  {mode}: {count}");
    }
}
