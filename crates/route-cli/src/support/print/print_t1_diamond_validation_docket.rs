//! Helper `print_t1_diamond_validation_docket`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_diamond_validation_docket(
    rows: &[T1DiamondValidationRow],
    priority: Option<&str>,
    source_rows: Option<&[T1SourceHealthRow]>,
    details: bool,
) {
    let tasks = t1_diamond_validation_tasks(rows, priority, source_rows);
    let mut by_category: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for task in &tasks {
        *by_category.entry(task.category.to_string()).or_insert(0) += 1;
    }

    println!("route t1-diamond-validation --docket");
    println!("  tasks: {} shown", tasks.len());
    println!("  categories: {}", format_count_map(&by_category));
    println!();
    println!(
        "{:<8} {:<20} {:<18} {:<14} {}",
        "Priority", "Category", "Site", "Intersection", "Action"
    );
    println!("{}", "-".repeat(132));
    for task in tasks {
        println!(
            "{:<8} {:<20} {:<18} {:<14} {}",
            task.priority_band, task.category, task.site_id, task.intersection, task.action
        );
        if details {
            println!("  location: {}", task.location);
            if let Some(source_action) = &task.source_action {
                println!("  source: {source_action}");
            }
        }
    }
}

