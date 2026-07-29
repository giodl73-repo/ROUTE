//! Helper `t2_relief_evidence_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_relief_evidence_rows(
    held_rows: &[T2HeldContactActionRow],
    bottleneck_rows: &[AtriBottleneckRow],
) -> Vec<T2ReliefEvidenceRow> {
    let mut bottlenecks_by_route =
        std::collections::BTreeMap::<String, Vec<&AtriBottleneckRow>>::new();
    for row in bottleneck_rows {
        bottlenecks_by_route
            .entry(canonical_route_key(&row.route))
            .or_default()
            .push(row);
    }
    for rows in bottlenecks_by_route.values_mut() {
        rows.sort_by_key(|row| row.rank);
    }

    let mut rows = held_rows
        .iter()
        .filter(|row| row.held_action_type == "relief-evidence-review")
        .map(|row| {
            let matches = bottlenecks_by_route
                .get(&canonical_route_key(&row.route))
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let annual_cost_m = matches.iter().map(|row| row.annual_cost_m).sum::<f64>();
            let top = matches.first();
            let (relief_action, evidence_basis, next_artifact, optimizer_effect) =
                if matches.is_empty() {
                    (
                        "source-gap-demote-or-find-evidence",
                        "no-atri-route-match",
                        "data/lower-tier-pressure-witnesses.csv",
                        "demote unless source-backed relief evidence is added",
                    )
                } else {
                    (
                        "source-observed-relief-review",
                        "atri-bottleneck-route-match",
                        "data/tier-contact-witnesses.csv",
                        "retain relief review only after contact repair validates",
                    )
                };

            T2ReliefEvidenceRow {
                route: row.route.clone(),
                source_exception_type: row.exception_type.clone(),
                bottleneck_match_count: matches.len(),
                top_bottleneck_rank: top.map(|row| row.rank).unwrap_or_default(),
                top_bottleneck_location: top
                    .map(|row| {
                        format!(
                            "{} ({}, {:.3}, {:.3})",
                            row.location, row.state, row.lat, row.lon
                        )
                    })
                    .unwrap_or_default(),
                annual_cost_m,
                relief_action: relief_action.to_string(),
                evidence_basis: evidence_basis.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    if rows.is_empty() {
        rows.push(T2ReliefEvidenceRow {
            route: "__all_t2_relief_evidence__".to_string(),
            source_exception_type: String::new(),
            bottleneck_match_count: 0,
            top_bottleneck_rank: 0,
            top_bottleneck_location: String::new(),
            annual_cost_m: 0.0,
            relief_action: "relief-evidence-clear".to_string(),
            evidence_basis: "no relief-evidence validation blockers remain".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "relief-evidence validation lane is clear".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}

