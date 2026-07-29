//! Helper `t3_zone_access_obligation_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_access_obligation_gate_failures(
    rows: &[T3ZoneAccessObligationRow],
    atlas_rows: &[MapAtlasRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T3 zone access obligations emitted".to_string());
        return failures;
    }

    let t3_maps = t3_zone_map_ids(atlas_rows);
    let covered_zones = rows
        .iter()
        .map(|row| row.zone_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    for map_id in &t3_maps {
        if !covered_zones.contains(map_id) {
            failures.push(format!("{map_id} has no access obligation row"));
        }
    }

    for row in rows {
        if row.zone_id.trim().is_empty()
            || row.zone_name.trim().is_empty()
            || row.obligation_class.trim().is_empty()
            || row.access_target.trim().is_empty()
            || row.source_route_count == 0
            || row.candidate_routes.trim().is_empty()
            || row.source_intake_classes.trim().is_empty()
            || row.map_id.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete access obligation fields",
                row.zone_id, row.obligation_class
            ));
        }
        if row.zone_id != row.map_id {
            failures.push(format!(
                "{} {} is detached from map id {}",
                row.zone_id, row.obligation_class, row.map_id
            ));
        }
        if !t3_maps.contains(&row.map_id) {
            failures.push(format!(
                "{} {} references unknown T3 map {}",
                row.zone_id, row.obligation_class, row.map_id
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} {} has invalid validation status {}",
                row.zone_id, row.obligation_class, row.validation_status
            ));
        }
    }
    failures
}

