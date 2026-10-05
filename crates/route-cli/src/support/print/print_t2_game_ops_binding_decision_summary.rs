//! Helper `print_t2_game_ops_binding_decision_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_game_ops_binding_decision_summary(
    output: &Path,
    rows: &[T2GameOpsBindingDecisionRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 game/ops binding decision rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}
