//! Helper `print_map_atlas`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_map_atlas(rows: &[MapAtlasRow], details: bool) {
    let failures = map_atlas_gate_failures(rows);
    let mut by_type = std::collections::BTreeMap::new();
    for row in rows {
        *by_type.entry(row.map_type.clone()).or_insert(0usize) += 1;
    }

    println!("route map-atlas");
    println!("  maps: {}", rows.len());
    println!("  types: {}", format_count_map(&by_type));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<18} {:<22} {:<14} {:<12} {}",
        "Map", "Path", "Type", "Contract", "Use"
    );
    println!("{}", "-".repeat(112));
    for row in rows {
        let contract = match png_dimensions(&map_atlas_artifact_path(&row.path)) {
            Some((width, height)) => format!("{width}x{height}"),
            None => "missing".to_string(),
        };
        println!(
            "{:<18} {:<22} {:<14} {:<12} {}",
            row.map_id,
            truncate_for_table(&row.path, 22),
            row.map_type,
            contract,
            row.tier_role
        );
        if details {
            println!("  command: {}", row.render_command);
            println!("  game: {}", row.game_use);
        }
    }
}

