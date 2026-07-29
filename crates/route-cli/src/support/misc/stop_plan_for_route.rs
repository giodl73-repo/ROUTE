//! Helper `stop_plan_for_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_plan_for_route<'a>(rows: &'a [StopCandidateRow], route: &str) -> Vec<&'a StopCandidateRow> {
    let mut stops = filter_stop_candidates(rows, None, Some(route));
    sort_stops_for_route(&mut stops);
    stops
}

