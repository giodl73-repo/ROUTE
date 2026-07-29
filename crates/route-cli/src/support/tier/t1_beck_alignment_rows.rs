//! Helper `t1_beck_alignment_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_beck_alignment_rows(stop_rows: &[T1StopSelectorInputRow]) -> Vec<T1BeckAlignmentRow> {
    let diagnostics = route_map::beck_t1_diagnostics()
        .into_iter()
        .map(|row| (normalise_designation(row.corridor), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut by_route = std::collections::BTreeMap::<String, Vec<&T1StopSelectorInputRow>>::new();
    for row in stop_rows {
        by_route
            .entry(normalise_designation(&row.route))
            .or_default()
            .push(row);
    }

    by_route
        .into_iter()
        .map(|(route, rows)| {
            let diagnostic = diagnostics.get(&route);
            let selector_stop_count = rows.len();
            let selector_boundary_count = rows.iter().filter(|row| row.boundary_after).count();
            let selector_regions = rows
                .iter()
                .map(|row| row.metis_region)
                .collect::<std::collections::BTreeSet<_>>()
                .len();
            let (beck_stop_count, beck_drawn_stop_count, beck_transfer_stop_count) = diagnostic
                .map_or((0, 0, 0), |row| {
                    (
                        row.stop_count,
                        row.drawn_stop_count,
                        row.transfer_stop_count,
                    )
                });
            let alignment_status = if diagnostic.is_none() {
                "missing-beck-route"
            } else if beck_stop_count < selector_stop_count {
                "beck-under-covers-selector"
            } else if diagnostic
                .map(|row| row.review_flag != "ok")
                .unwrap_or(false)
            {
                "aligned-with-policy-review"
            } else {
                "aligned"
            };
            T1BeckAlignmentRow {
                route,
                selector_stop_count,
                selector_boundary_count,
                selector_regions,
                beck_stop_count,
                beck_drawn_stop_count,
                beck_transfer_stop_count,
                beck_action: diagnostic
                    .map(|row| row.service_action.to_string())
                    .unwrap_or_else(|| "missing-diagnostic".to_string()),
                beck_review_flag: diagnostic
                    .map(|row| row.review_flag.to_string())
                    .unwrap_or_else(|| "missing-diagnostic".to_string()),
                alignment_status: alignment_status.to_string(),
                validation_status: if matches!(
                    alignment_status,
                    "aligned" | "aligned-with-policy-review"
                ) {
                    "pass"
                } else {
                    "review"
                }
                .to_string(),
            }
        })
        .collect()
}

