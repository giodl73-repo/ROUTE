//! Helper `print_tier_contact_witness_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_contact_witness_summary(output: &Path, rows: &[TierContactWitnessRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.witness_type.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} witness rows to {}",
        rows.len(),
        output.display()
    );
    for (witness_type, count) in counts {
        println!("  {witness_type}: {count}");
    }
}

