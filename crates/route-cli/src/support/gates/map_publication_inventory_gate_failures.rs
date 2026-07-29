//! Helper `map_publication_inventory_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn map_publication_inventory_gate_failures(
    inventory_rows: &[MapPublicationInventoryRow],
    atlas_rows: &[MapAtlasRow],
    readiness_rows: &[MapPublicationReadinessRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if inventory_rows.is_empty() {
        failures.push("map publication inventory has no rows".to_string());
    }
    let Some(readiness) = readiness_rows.first() else {
        failures.push("map publication readiness has no rows".to_string());
        return failures;
    };
    if readiness.validation_status != "pass" {
        failures.push(format!(
            "map publication readiness is {}",
            readiness.validation_status
        ));
    }
    if readiness.publication_blocker_count != 0 {
        failures.push(format!(
            "map publication readiness has {} publication blockers",
            readiness.publication_blocker_count
        ));
    }
    if inventory_rows.len() != atlas_rows.len() {
        failures.push(format!(
            "inventory row count {} != atlas row count {}",
            inventory_rows.len(),
            atlas_rows.len()
        ));
    }

    let atlas_by_id = atlas_rows
        .iter()
        .map(|row| (row.map_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut seen_ids = std::collections::BTreeSet::new();
    let required_forbidden = [
        "evidence-valid",
        "sla-valid",
        "transit-ready",
        "upgrade-ready",
        "asset-condition-repaired",
    ];

    for row in inventory_rows {
        if !seen_ids.insert(row.map_id.as_str()) {
            failures.push(format!("{} appears more than once", row.map_id));
        }
        let Some(atlas) = atlas_by_id.get(row.map_id.as_str()) else {
            failures.push(format!("{} is not in map atlas", row.map_id));
            continue;
        };
        if row.map_path != atlas.path {
            failures.push(format!(
                "{} path {} != atlas {}",
                row.map_id, row.map_path, atlas.path
            ));
        }
        if row.map_type != atlas.map_type {
            failures.push(format!(
                "{} type {} != atlas {}",
                row.map_id, row.map_type, atlas.map_type
            ));
        }
        if row.publication_status != "publication-ready-held-claims" {
            failures.push(format!(
                "{} publication status is {}",
                row.map_id, row.publication_status
            ));
        }
        if row.render_gate_status != "pass" || row.validation_status != "pass" {
            failures.push(format!("{} inventory status is not pass", row.map_id));
        }
        if row.readiness_artifact != "data/map-publication-readiness.csv" {
            failures.push(format!("{} points to wrong readiness artifact", row.map_id));
        }
        if split_claim_tokens(&row.held_claims)
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
            != split_claim_tokens(&readiness.held_claims)
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
        {
            failures.push(format!(
                "{} held claims {} != readiness {}",
                row.map_id, row.held_claims, readiness.held_claims
            ));
        }
        if !row.required_label.contains("held") {
            failures.push(format!(
                "{} label does not identify held claims",
                row.map_id
            ));
        }
        for forbidden in required_forbidden {
            if !split_claim_tokens(&row.not_allowed_claims)
                .iter()
                .any(|claim| *claim == forbidden)
            {
                failures.push(format!("{} does not forbid {}", row.map_id, forbidden));
            }
        }
    }

    for atlas in atlas_rows {
        if !seen_ids.contains(atlas.map_id.as_str()) {
            failures.push(format!(
                "{} missing from publication inventory",
                atlas.map_id
            ));
        }
    }

    failures
}

