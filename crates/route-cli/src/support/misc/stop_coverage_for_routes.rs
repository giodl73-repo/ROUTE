//! Helper `stop_coverage_for_routes`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_coverage_for_routes(
    rows: &[StopCandidateRow],
    routes: &[String],
    tier: &str,
) -> Vec<StopCoverageRow> {
    routes
        .iter()
        .map(|route| {
            let plan = stop_plan_for_route(rows, route);
            let mut by_class = std::collections::BTreeMap::new();
            for stop in &plan {
                *by_class
                    .entry(stop.requested_class.trim().to_ascii_uppercase())
                    .or_insert(0usize) += 1;
            }
            let major_stop_count = plan
                .iter()
                .filter(|stop| {
                    matches!(
                        stop.requested_class.trim().to_ascii_uppercase().as_str(),
                        "S1" | "S2"
                    )
                })
                .count();
            StopCoverageRow {
                route: route.clone(),
                stop_count: plan.len(),
                major_stop_count,
                classes: format_count_map(&by_class),
                failures: stop_plan_gate_failures_for_tier(route, &plan, tier),
            }
        })
        .collect()
}
