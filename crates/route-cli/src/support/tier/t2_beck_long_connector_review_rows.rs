//! Helper `t2_beck_long_connector_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_beck_long_connector_review_rows(
    claim_rows: &[OptimizerClaimReviewRow],
    diagnostics: &[route_map::BeckT2DiagnosticRow],
) -> Vec<T2BeckLongConnectorReviewRow> {
    let Some(claim_row) = claim_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T2"
            && row.blocker_family == "beck_long_connector"
            && row.total_claim_blockers > 0
    }) else {
        return Vec::new();
    };
    let expected_routes = claim_row
        .representative_routes
        .split(';')
        .filter(|route| !route.trim().is_empty())
        .map(route_display_key)
        .collect::<std::collections::BTreeSet<_>>();
    let mut rows = diagnostics
        .iter()
        .filter(|row| {
            row.review_flag == "long-connector-review"
                && expected_routes.contains(&route_display_key(row.corridor))
        })
        .map(|row| T2BeckLongConnectorReviewRow {
            connector_review_id: format!("T2BECKLONG-{}", stable_id_fragment(row.corridor)),
            claim_review_id: claim_row.claim_review_id.clone(),
            route: route_display_key(row.corridor),
            trunk: route_display_key(row.trunk),
            start_trunk: route_display_key(row.start_trunk),
            end_trunk: route_display_key(row.end_trunk),
            service_class: row.service_class.to_string(),
            service_label: row.service_label.to_string(),
            stop_count: row.stop_count,
            transfer_stop_count: row.transfer_stop_count,
            schematic_length_px: row.schematic_length_px,
            split_anchor: row.split_anchor.to_string(),
            split_anchor_offset_pct: row.split_anchor_offset_pct,
            review_flag: row.review_flag.to_string(),
            connector_basis: format!(
                "schematic_length_px={:.0};stops={};split_anchor_offset_pct={:.0}",
                row.schematic_length_px, row.stop_count, row.split_anchor_offset_pct
            ),
            review_decision: "long-connector-policy-required".to_string(),
            blocker_claims_before: claim_row.blocked_claims.clone(),
            blocker_claims_after: claim_row.blocked_claims.clone(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-long-connector-policy.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
