//! Helper `print_t1_design_policy`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_design_policy(
    review_rows: &[T1DesignReviewCsvRow],
    policy_rows: &[T1DesignPolicyActionRow],
    details: bool,
) {
    let mut action_counts = std::collections::BTreeMap::<String, usize>::new();
    for row in review_rows {
        *action_counts
            .entry(row.next_design_action.clone())
            .or_insert(0) += 1;
    }

    println!("route t1-design-policy");
    println!("  review rows: {}", review_rows.len());
    println!("  policy actions: {}", policy_rows.len());
    println!("  action use: {}", format_count_map(&action_counts));
    println!();
    println!("{:<34} {:<18} {:>5} Treatment", "Action", "Status", "Uses");
    println!("{}", "-".repeat(110));
    for row in policy_rows {
        let uses = action_counts.get(&row.action).copied().unwrap_or(0);
        println!(
            "{:<34} {:<18} {:>5} {}",
            row.action, row.applies_to_status, uses, row.design_treatment
        );
        if details {
            println!("  definition: {}", row.definition);
            println!("  required_policy: {}", row.required_policy);
            println!("  gate_policy: {}", row.gate_policy);
            println!("  next_selector_use: {}", row.next_selector_use);
        }
    }
}
