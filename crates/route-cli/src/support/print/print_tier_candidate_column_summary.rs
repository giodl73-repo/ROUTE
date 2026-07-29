//! Helper `print_tier_candidate_column_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_candidate_column_summary(output: &Path, rows: &[TierCandidateColumnRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.column_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} candidate column rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

