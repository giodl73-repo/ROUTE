//! Helper `map_atlas_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn map_atlas_gate_failures(rows: &[MapAtlasRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("map atlas has no rows".to_string());
    }
    for row in rows {
        if row.map_id.trim().is_empty()
            || row.path.trim().is_empty()
            || row.map_type.trim().is_empty()
            || row.render_command.trim().is_empty()
            || row.tier_role.trim().is_empty()
            || row.game_use.trim().is_empty()
        {
            failures.push(format!("{} has empty manifest fields", row.map_id));
            continue;
        }
        let path = map_atlas_artifact_path(&row.path);
        let Ok(metadata) = std::fs::metadata(&path) else {
            failures.push(format!("{} missing {}", row.map_id, row.path));
            continue;
        };
        if metadata.len() < row.min_bytes {
            failures.push(format!(
                "{} too small: {} bytes < {}",
                row.map_id,
                metadata.len(),
                row.min_bytes
            ));
        }
        match png_dimensions(&path) {
            Some((width, height))
                if width == row.expected_width && height == row.expected_height => {}
            Some((width, height)) => failures.push(format!(
                "{} dimensions {}x{} != {}x{}",
                row.map_id, width, height, row.expected_width, row.expected_height
            )),
            None => failures.push(format!("{} is not a readable PNG", row.map_id)),
        }
    }
    failures
}

