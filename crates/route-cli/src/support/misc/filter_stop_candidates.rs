//! Helper `filter_stop_candidates`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn filter_stop_candidates<'a>(
    rows: &'a [StopCandidateRow],
    stop_class: Option<&str>,
    route: Option<&str>,
) -> Vec<&'a StopCandidateRow> {
    let route = route.map(normalise_designation);
    rows.iter()
        .filter(|row| {
            stop_class
                .map(|class| {
                    row.requested_class
                        .trim()
                        .eq_ignore_ascii_case(class.trim())
                })
                .unwrap_or(true)
        })
        .filter(|row| {
            route
                .as_ref()
                .map(|route| {
                    stop_candidate_routes(row)
                        .iter()
                        .any(|candidate| candidate == route)
                })
                .unwrap_or(true)
        })
        .collect()
}
