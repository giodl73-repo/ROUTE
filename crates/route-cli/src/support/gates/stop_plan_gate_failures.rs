//! Helper `stop_plan_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_plan_gate_failures(route: &str, stops: &[&StopCandidateRow]) -> Vec<String> {
    stop_plan_gate_failures_for_tier(route, stops, "T1")
}

