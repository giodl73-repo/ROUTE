//! Helper `stop_plan_gate_failures_for_tier`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_plan_gate_failures_for_tier(
    route: &str,
    stops: &[&StopCandidateRow],
    tier: &str,
) -> Vec<String> {
    let mut failures = Vec::new();
    if stops.is_empty() {
        failures.push(format!("{route}: no stop candidates"));
        return failures;
    }
    let tier = tier.trim().to_ascii_uppercase();
    let min_stops = match tier.as_str() {
        "T2" | "T3" => 2,
        _ => 3,
    };
    let min_endpoint_grade = match tier.as_str() {
        "T2" | "T3" => 1,
        _ => 2,
    };

    if stops.len() < min_stops {
        failures.push(format!(
            "{route}: only {} stop candidate(s); tier schematic lines need a visible chain",
            stops.len()
        ));
    }
    let endpoint_grade_count = stops
        .iter()
        .filter(|stop| {
            let class = stop.requested_class.trim().to_ascii_uppercase();
            if tier == "T3" {
                matches!(class.as_str(), "S1" | "S2" | "S3")
            } else {
                matches!(class.as_str(), "S1" | "S2")
            }
        })
        .count();
    if endpoint_grade_count < min_endpoint_grade {
        failures.push(format!(
            "{route}: needs at least {min_endpoint_grade} terminal or transfer-grade stop(s)"
        ));
    }
    failures.extend(stop_candidate_gate_failures(stops));
    failures
}

