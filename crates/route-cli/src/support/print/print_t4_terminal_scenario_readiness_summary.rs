//! Helper `print_t4_terminal_scenario_readiness_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_scenario_readiness_summary(
    output: &Path,
    rows: &[T4TerminalScenarioReadinessRow],
) {
    let mut by_decision = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_decision
            .entry(row.scenario_decision.as_str())
            .or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal scenario readiness rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in by_decision {
        println!("  {decision}: {count}");
    }
}
