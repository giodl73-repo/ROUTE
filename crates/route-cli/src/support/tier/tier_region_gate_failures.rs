//! Helper `tier_region_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_region_gate_failures(
    rows: &[TierRegionWorkloadRow],
    requested_regions: usize,
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no tier region workload rows emitted".to_string());
        return failures;
    }
    if let Some(status) = rows
        .first()
        .map(|row| row.component_status.as_str())
        .filter(|status| status.starts_with("component-bridged:"))
    {
        failures.push(format!(
            "dual route graph required {status}; repair route contacts before gate can pass"
        ));
    }
    let mut route_counts = vec![0usize; requested_regions];
    for row in rows {
        if row.region_id >= requested_regions {
            failures.push(format!(
                "{} assigned to out-of-range region {}",
                row.route, row.region_id
            ));
            continue;
        }
        if row.route_weight <= 0 {
            failures.push(format!("{} has non-positive route weight", row.route));
        }
        if row.component_status == "connected"
            && !row.validation_status.eq_ignore_ascii_case("pass")
        {
            failures.push(format!(
                "{} has validation_status={}",
                row.route, row.validation_status
            ));
        }
        route_counts[row.region_id] += 1;
    }
    for (region, count) in route_counts.into_iter().enumerate() {
        if count == 0 {
            failures.push(format!("region {region} has no assigned routes"));
        }
    }
    failures
}
