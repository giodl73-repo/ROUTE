//! Helper `print_t2_bundle_readiness_replay_decision_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_bundle_readiness_replay_decision_summary(
    output: &Path,
    rows: &[T2BundleReadinessReplayDecisionRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.replay_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle readiness replay decision rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

