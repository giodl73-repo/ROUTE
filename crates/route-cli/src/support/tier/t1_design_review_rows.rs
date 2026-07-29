//! Helper `t1_design_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_design_review_rows(selector_rows: &[T1LineSelectorRow]) -> Vec<T1DesignReviewRow> {
    let diagnostics = route_map::beck_t1_diagnostics()
        .into_iter()
        .map(|row| (normalise_designation(row.corridor), row))
        .collect::<std::collections::BTreeMap<_, _>>();

    selector_rows
        .iter()
        .filter(|row| {
            row.selected
                || row.sla_pair_count > 0
                || row.decision == "reject-score-exception"
                || row.decision == "reject-route-budget"
                || row.decision == "reject-stop-budget"
        })
        .map(|row| {
            let diagnostic = diagnostics.get(&row.route);
            let beck_action = diagnostic
                .map(|diag| diag.service_action.to_string())
                .unwrap_or_else(|| "missing-diagnostic".to_string());
            let beck_review_flag = diagnostic
                .map(|diag| diag.review_flag.to_string())
                .unwrap_or_else(|| "missing-diagnostic".to_string());
            let overlap_corridors = diagnostic
                .map(|diag| diag.shared_segment_corridors.clone())
                .unwrap_or_default();
            let design_role = if row.selected && row.sla_pair_count > 0 {
                "promise-spine"
            } else if row.selected {
                "score-backbone-exception"
            } else if row.decision == "reject-score-exception" {
                "score-backbone-demoted"
            } else if row.sla_pair_count > 0 {
                "unmet-promise-blocker"
            } else {
                "cutline-candidate"
            };
            let explicit_score_exception_keep = matches!(
                row.reason,
                "score-exception-keep" | "score-exception-conditional-keep"
            );
            let design_status = if !row.selected {
                "held"
            } else if diagnostic.is_none()
                || (row.sla_pair_count == 0 && !explicit_score_exception_keep)
                || beck_review_flag != "ok"
            {
                "policy-review"
            } else {
                "accepted"
            };
            let next_design_action = if !row.selected && row.sla_pair_count > 0 {
                "raise-budget-or-recut-promises"
            } else if row.decision == "reject-score-exception" {
                "score-exception-demoted"
            } else if !row.selected {
                "hold-outside-current-budget"
            } else if diagnostic.is_none() {
                "add-beck-diagnostic-row"
            } else if row.sla_pair_count == 0 && !explicit_score_exception_keep {
                "justify-as-national-relay-or-demote-to-t2"
            } else if beck_review_flag != "ok" {
                "resolve-shared-segment-map-policy"
            } else {
                "keep-in-t1-design"
            };

            T1DesignReviewRow {
                route: row.route.clone(),
                selected: row.selected,
                design_role,
                promise_count: row.sla_pair_count,
                selected_stop_count: row.selected_stop_count,
                top_city_stop_count: row.top_city_stop_count,
                selector_reason: row.reason.to_string(),
                beck_action,
                beck_review_flag,
                overlap_corridors,
                design_status,
                next_design_action,
            }
        })
        .collect()
}

