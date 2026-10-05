//! Helper `t1_stop_selector_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_stop_selector_gate_failures(rows: &[T1StopSelectorRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T1 stop selector rows emitted".to_string());
        return failures;
    }
    let mut by_route = std::collections::BTreeMap::<&str, Vec<&T1StopSelectorRow>>::new();
    for row in rows {
        by_route.entry(row.route.as_str()).or_default().push(row);
        if row.selector_weight <= 0 {
            failures.push(format!(
                "{}:{} has non-positive selector weight",
                row.route, row.stop_id
            ));
        }
        if !row.validation_status.eq_ignore_ascii_case("pass") {
            failures.push(format!(
                "{}:{} has validation_status={}",
                row.route, row.stop_id, row.validation_status
            ));
        }
    }
    for (route, route_rows) in by_route {
        if route_rows.len() < 3 {
            failures.push(format!("{route}: fewer than 3 selected stops"));
        }
        let regions = route_rows
            .iter()
            .map(|row| row.metis_region)
            .collect::<std::collections::BTreeSet<_>>();
        if regions.len() != route_rows[0].target_regions {
            failures.push(format!(
                "{route}: expected {} METIS regions, found {}",
                route_rows[0].target_regions,
                regions.len()
            ));
        }
    }
    failures
}
