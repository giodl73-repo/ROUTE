//! Helper `print_t1_score_exceptions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_score_exceptions(
    review_rows: &[T1DesignReviewCsvRow],
    exception_rows: &[T1ScoreExceptionRow],
    details: bool,
) {
    let score_only_count = review_rows
        .iter()
        .filter(|row| row.selected && row.design_role == "score-backbone-exception")
        .count();
    let mut by_decision = std::collections::BTreeMap::<String, usize>::new();
    for row in exception_rows {
        *by_decision.entry(row.decision.clone()).or_insert(0) += 1;
    }

    println!("route t1-score-exceptions");
    println!("  score-only selected T1 routes: {score_only_count}");
    println!("  exception rows: {}", exception_rows.len());
    println!("  decisions: {}", format_count_map(&by_decision));
    println!();
    println!(
        "{:<8} {:<24} {:<22} Replacement",
        "Route", "Decision", "Exception"
    );
    println!("{}", "-".repeat(92));
    for row in exception_rows {
        println!(
            "{:<8} {:<24} {:<22} {}",
            row.route, row.decision, row.exception_type, row.replacement_candidate
        );
        if details {
            println!("  rationale: {}", row.rationale);
            println!("  evidence: {}", row.evidence_status);
            println!("  artifact: {}", row.artifact);
            println!("  next: {}", row.next_selector_action);
        }
    }
}

