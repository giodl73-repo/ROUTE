//! Helper `t3_zone_map_diagnostic_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_map_diagnostic_gate_failures(
    rows: &[T3ZoneMapDiagnosticRow],
    atlas_rows: &[MapAtlasRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T3 zone map diagnostics emitted".to_string());
        return failures;
    }

    let t3_maps = t3_zone_map_ids(atlas_rows);
    let row_maps = rows
        .iter()
        .map(|row| row.map_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    for map_id in &t3_maps {
        if !row_maps.contains(map_id) {
            failures.push(format!("{map_id} has no map diagnostic row"));
        }
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.zone_id.trim().is_empty()
            || row.zone_name.trim().is_empty()
            || row.map_id.trim().is_empty()
            || row.map_path.trim().is_empty()
            || row.map_readiness.trim().is_empty()
            || row.diagnostic_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete map diagnostic fields",
                row.zone_id
            ));
        }
        if !seen.insert(row.zone_id.clone()) {
            failures.push(format!("{} has duplicate map diagnostic row", row.zone_id));
        }
        if row.zone_id != row.map_id {
            failures.push(format!(
                "{} is detached from map id {}",
                row.zone_id, row.map_id
            ));
        }
        if !t3_maps.contains(&row.map_id) {
            failures.push(format!("{} references unknown T3 map", row.zone_id));
        }
        if row.selected_route_count == 0 {
            failures.push(format!("{} has no selected T3 feeder routes", row.zone_id));
        }
        if row.selected_route_count > 0 && row.selected_routes.trim().is_empty() {
            failures.push(format!(
                "{} has selected count but no route list",
                row.zone_id
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.zone_id, row.validation_status
            ));
        }
    }

    failures
}

