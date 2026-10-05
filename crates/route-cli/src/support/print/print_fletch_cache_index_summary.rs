//! Helper `print_fletch_cache_index_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_fletch_cache_index_summary(
    output: &Path,
    report: &route_data::FletchCacheIndexReport,
    details: bool,
) {
    println!(
        "  wrote {} FLETCH cache-index rows to {}",
        report.rows.len(),
        output.display()
    );
    println!("  registry: {}", report.registry_id);
    println!(
        "  registered matched: {}/{} (missing: {})",
        report.matched_registered_count, report.registered_count, report.missing_registered_count
    );
    println!(
        "  entries: {} verified / {} unverified; unexpected: {}; bytes: {}",
        report.verified_count,
        report.unverified_count,
        report.unexpected_entry_count,
        report.byte_count
    );
    if details {
        println!();
        println!("  {:36}  {:10}  {:10}  path", "FLETCH", "registry", "cache");
        println!("  {}", "-".repeat(92));
        for row in &report.rows {
            println!(
                "  {:36}  {:10}  {:10}  {}",
                row.fletch_id, row.registry_status, row.cache_status, row.relative_path
            );
        }
    }
}
