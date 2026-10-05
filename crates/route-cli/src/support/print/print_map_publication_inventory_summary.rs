//! Helper `print_map_publication_inventory_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_map_publication_inventory_summary(
    path: &Path,
    rows: &[MapPublicationInventoryRow],
    details: bool,
) {
    let mut by_type = std::collections::BTreeMap::new();
    for row in rows {
        *by_type.entry(row.map_type.clone()).or_insert(0usize) += 1;
    }
    println!("  inventory: {}", path.display());
    println!("  maps: {}", rows.len());
    println!("  types: {}", format_count_map(&by_type));
    if details {
        for row in rows {
            println!(
                "  {} -> {} ({}) {}",
                row.map_id, row.map_path, row.map_type, row.publication_status
            );
        }
    }
}
