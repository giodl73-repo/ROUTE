//! Helper `print_fletch_source_handoff_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_fletch_source_handoff_summary(
    output: &Path,
    report: &route_data::FletchSourceHandoffReport,
    details: bool,
) {
    let mut statuses = std::collections::BTreeMap::<&str, usize>::new();
    for row in &report.rows {
        *statuses.entry(row.handoff_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} FLETCH source handoff rows to {}",
        report.rows.len(),
        output.display()
    );
    println!("  registry: {}", report.registry_id);
    println!(
        "  families covered: {}/{}",
        report.covered_family_count, report.policy_family_count
    );
    println!(
        "  fletches: {}  sources: {}  adapter-owned: {}",
        report.fletch_count, report.source_count, report.adapter_source_count
    );
    println!(
        "  graph: {} nodes / {} edges; dry-run steps: {}",
        report.graph_node_count, report.graph_edge_count, report.flight_step_count
    );
    for (status, count) in statuses {
        println!("  {status}: {count}");
    }
    if !report.missing_policy_families.is_empty() {
        println!(
            "  missing policy families: {}",
            report.missing_policy_families.join(", ")
        );
    }
    if details {
        println!();
        println!("  {:28}  {:18}  {}", "FLETCH", "handoff", "target");
        println!("  {}", "-".repeat(76));
        for row in &report.rows {
            println!(
                "  {:28}  {:18}  {}",
                row.fletch_id, row.handoff_status, row.cache_targets
            );
        }
    }
}
