//! Helper `print_t1_access_docket`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_access_docket(rows: &[T1SourceHealthRow], category: Option<&str>, details: bool) {
    let mut docket = rows
        .iter()
        .filter(|row| t1_source_health_is_blocked(row))
        .map(t1_access_docket_item)
        .filter(|item| {
            category
                .map(|category| item.category.eq_ignore_ascii_case(category))
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();
    docket.sort_by(|a, b| {
        t1_access_priority_rank(&a.priority)
            .cmp(&t1_access_priority_rank(&b.priority))
            .then_with(|| a.category.cmp(&b.category))
            .then_with(|| a.site_id.cmp(&b.site_id))
    });

    let mut by_category: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for item in &docket {
        *by_category.entry(item.category.clone()).or_insert(0) += 1;
    }

    println!("route t1-access-docket");
    println!("  actions: {} shown", docket.len());
    println!("  categories: {}", format_count_map(&by_category));
    println!();
    println!(
        "{:<10} {:<16} {:<18} {:<24} {}",
        "Priority", "Category", "Site", "Source", "Action"
    );
    println!("{}", "-".repeat(132));
    for item in docket {
        println!(
            "{:<10} {:<16} {:<18} {:<24} {}",
            item.priority,
            item.category,
            item.site_id,
            truncate_for_table(&item.source_name, 24),
            item.action
        );
        if details {
            println!("  access: {}", item.access_health);
            println!("  history: {}", item.history_status);
            println!("  url: {}", item.source_url);
            println!("  gap: {}", item.blocking_gap);
        }
    }
}

