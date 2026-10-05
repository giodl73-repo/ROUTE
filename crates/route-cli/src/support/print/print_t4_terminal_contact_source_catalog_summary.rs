//! Helper `print_t4_terminal_contact_source_catalog_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_contact_source_catalog_summary(
    output: &Path,
    rows: &[T4TerminalContactSourceCatalogRow],
) {
    println!(
        "  wrote {} T4 terminal contact source catalog rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {}: {} route tasks ({})",
            row.terminal_district, row.route_task_count, row.acquisition_status
        );
    }
}
