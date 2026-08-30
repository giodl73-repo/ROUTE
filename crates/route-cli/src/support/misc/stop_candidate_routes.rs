//! Helper `stop_candidate_routes`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_candidate_routes(row: &StopCandidateRow) -> Vec<String> {
    row.route_refs
        .split([';', ','])
        .map(|route| normalise_designation(route.trim()))
        .filter(|route| !route.is_empty())
        .collect()
}
