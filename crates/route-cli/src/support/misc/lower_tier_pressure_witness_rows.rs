//! Helper `lower_tier_pressure_witness_rows` (support::misc).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn lower_tier_pressure_witness_rows(
    tier_rows: &[TierTableScoreRow],
    candidate_rows: &[TierCandidateColumnRow],
    resolution_rows: &[T2ContactResolutionRow],
    dispositions: &std::collections::HashMap<String, T2ClosureDisposition>,
) -> Vec<LowerTierPressureWitnessRow> {
    let mut rows = Vec::new();
    let tier_row_by_route = tier_rows
        .iter()
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::HashMap<_, _>>();

    for row in candidate_rows
        .iter()
        .filter(|row| row.tier == "T2" && row.column_decision == "demote")
    {
        let score_row = tier_row_by_route.get(&canonical_route_key(&row.route));
        rows.push(LowerTierPressureWitnessRow {
            route: row.route.clone(),
            current_tier: row.tier.clone(),
            current_score: score_row
                .map(|score_row| score_row.score)
                .unwrap_or_default(),
            confidence: score_row
                .map(|score_row| score_row.confidence)
                .unwrap_or_default(),
            confidence_label: score_row
                .map(|score_row| score_row.confidence_label.clone())
                .unwrap_or_else(|| "n/a".to_string()),
            pressure_type: "demotion-pressure".to_string(),
            witness_action: "demote-to-lower-tier-treatment".to_string(),
            target_tier: "T3/T4".to_string(),
            selection_basis: row.repair_basis.clone(),
            source_artifact: "data/tier-candidate-columns.csv".to_string(),
            next_artifact: "data/tier-table.csv".to_string(),
            validation_status: "review".to_string(),
        });
    }

    let existing_routes = rows
        .iter()
        .map(|row| canonical_route_key(&row.route))
        .collect::<std::collections::BTreeSet<_>>();
    for row in resolution_rows
        .iter()
        .filter(|row| row.resolution_action == "move-to-lower-tier-pressure")
        .filter(|row| !existing_routes.contains(&canonical_route_key(&row.route)))
    {
        let score_row = tier_row_by_route.get(&canonical_route_key(&row.route));
        rows.push(LowerTierPressureWitnessRow {
            route: row.route.clone(),
            current_tier: "T2".to_string(),
            current_score: score_row
                .map(|score_row| score_row.score)
                .unwrap_or_default(),
            confidence: score_row
                .map(|score_row| score_row.confidence)
                .unwrap_or_default(),
            confidence_label: score_row
                .map(|score_row| score_row.confidence_label.clone())
                .unwrap_or_else(|| "n/a".to_string()),
            pressure_type: "demotion-pressure".to_string(),
            witness_action: "demote-to-lower-tier-treatment".to_string(),
            target_tier: "T3/T4".to_string(),
            selection_basis: row.resolution_basis.clone(),
            source_artifact: "data/t2-contact-resolutions.csv".to_string(),
            next_artifact: "data/tier-table.csv".to_string(),
            validation_status: "review".to_string(),
        });
    }

    let existing_routes = rows
        .iter()
        .map(|row| canonical_route_key(&row.route))
        .collect::<std::collections::BTreeSet<_>>();
    let mut closure_pressure_rows = dispositions
        .values()
        .filter(|row| row.disposition == "lower-tier-pressure")
        .filter(|row| !existing_routes.contains(&canonical_route_key(&row.route)))
        .collect::<Vec<_>>();
    closure_pressure_rows.sort_by(|a, b| a.route.cmp(&b.route));
    for row in closure_pressure_rows {
        let score_row = tier_row_by_route.get(&canonical_route_key(&row.route));
        rows.push(LowerTierPressureWitnessRow {
            route: row.route.clone(),
            current_tier: "T2".to_string(),
            current_score: score_row
                .map(|score_row| score_row.score)
                .unwrap_or_default(),
            confidence: score_row
                .map(|score_row| score_row.confidence)
                .unwrap_or_default(),
            confidence_label: score_row
                .map(|score_row| score_row.confidence_label.clone())
                .unwrap_or_else(|| "n/a".to_string()),
            pressure_type: "closure-demotion-pressure".to_string(),
            witness_action: row.action.clone(),
            target_tier: "T3/T4".to_string(),
            selection_basis: row.basis.clone(),
            source_artifact: row.source_artifact.clone(),
            next_artifact: row.next_artifact.clone(),
            validation_status: "review".to_string(),
        });
    }

    for row in tier_rows {
        if row.tier == "T3" && row.score >= T2_THRESHOLD - 5.0 {
            rows.push(lower_tier_score_pressure_row(
                row,
                "regional-upgrade-pressure",
                "evaluate-for-t2-upgrade-candidate",
                "T2",
                "score-within-five-points-of-t2-threshold",
            ));
        } else if row.tier == "T4" && row.score >= T3_THRESHOLD - 5.0 {
            rows.push(lower_tier_score_pressure_row(
                row,
                "local-upgrade-pressure",
                "evaluate-for-t3-access-candidate",
                "T3",
                "score-within-five-points-of-t3-threshold",
            ));
        }
    }

    rows.sort_by(|a, b| {
        a.current_tier
            .cmp(&b.current_tier)
            .then_with(|| b.current_score.total_cmp(&a.current_score))
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}
