//! Helper `map_publication_readiness_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn map_publication_readiness_rows(
    atlas_rows: &[MapAtlasRow],
    backlog_rows: &[OptimizerResidualBlockerBacklogRow],
    scope_rows: &[MapPublicationScopeDecisionRow],
    scope_decision_path: &Path,
    backlog_path: &Path,
) -> Vec<MapPublicationReadinessRow> {
    let render_gate_status = if map_atlas_gate_failures(atlas_rows).is_empty() {
        "pass"
    } else {
        "blocked"
    };
    let scope_decision_status = scope_rows
        .iter()
        .find(|row| row.decision_id == "MAPPUB-FULL-T1T4")
        .map(|row| row.validation_status.as_str())
        .unwrap_or("missing");

    let mut map_types = std::collections::BTreeSet::new();
    for row in atlas_rows {
        map_types.insert(row.map_type.as_str());
    }

    let mut publication_blocker_count = 0usize;
    let mut publication_blocker_families = std::collections::BTreeSet::new();
    let mut held_claims = std::collections::BTreeSet::new();
    let mut held_claim_family_count = 0usize;
    let mut budget_debt_count = 0usize;

    for row in backlog_rows {
        budget_debt_count += row.total_budget_debt_count;
        let claims = split_claim_tokens(&row.blocked_claims);
        if claims.iter().any(|claim| *claim == "publication") {
            publication_blocker_count += row.total_claim_blockers + row.total_budget_debt_count;
            publication_blocker_families.insert(row.blocker_family.as_str());
        }
        let mut held_for_row = false;
        for claim in claims {
            if claim != "publication" {
                held_claims.insert(claim);
                held_for_row = true;
            }
        }
        if held_for_row {
            held_claim_family_count += 1;
        }
    }

    let validation_status = if render_gate_status == "pass"
        && publication_blocker_count == 0
        && scope_decision_status == "pass"
    {
        "pass"
    } else {
        "blocked"
    };

    vec![MapPublicationReadinessRow {
        readiness_id: "MAPPUB-READY-T1T4-STRUCTURAL".to_string(),
        map_surface: "T1-T4 structural maps".to_string(),
        map_count: atlas_rows.len(),
        map_types: map_types.into_iter().collect::<Vec<_>>().join(";"),
        render_gate_status: render_gate_status.to_string(),
        scope_decision_status: scope_decision_status.to_string(),
        publication_blocker_count,
        publication_blocker_families: publication_blocker_families
            .into_iter()
            .collect::<Vec<_>>()
            .join(";"),
        held_claims: held_claims.into_iter().collect::<Vec<_>>().join(";"),
        held_claim_family_count,
        budget_debt_count,
        scope_decision_artifact: scope_decision_path.display().to_string(),
        backlog_artifact: backlog_path.display().to_string(),
        readiness_decision: if validation_status == "pass" {
            "publish-structural-t1-t4-maps-with-held-claim-labels".to_string()
        } else {
            "hold-map-publication-until-render-scope-and-publication-blockers-pass".to_string()
        },
        next_artifact: "docs/map-publication-scope.md".to_string(),
        validation_status: validation_status.to_string(),
    }]
}
