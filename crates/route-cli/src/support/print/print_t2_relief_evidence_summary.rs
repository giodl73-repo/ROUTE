//! Helper `print_t2_relief_evidence_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_relief_evidence_summary(output: &Path, rows: &[T2ReliefEvidenceRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.relief_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} relief evidence rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

