//! Helper `t3_zone_stop_plan_for_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_stop_plan_for_route<'a>(
    rows: &'a [StopCandidateRow],
    route: &str,
    zone_id: &str,
) -> Vec<&'a StopCandidateRow> {
    let mut stops = stop_plan_for_route(rows, route)
        .into_iter()
        .filter(|stop| t3_stop_in_zone(stop, zone_id))
        .collect::<Vec<_>>();
    sort_stops_for_route(&mut stops);
    stops
}

