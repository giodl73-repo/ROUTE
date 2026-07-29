//! Helper `t1_failure_event_has_observation_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_failure_event_has_observation_contract(row: &T1FailureEventRow) -> bool {
    let confidence = row.confidence.trim().to_ascii_lowercase();
    let confidence_is_labeled = matches!(confidence.as_str(), "high" | "medium" | "low");
    let has_timing = row.duration_hours.is_some()
        || (!row.start_time.trim().is_empty() && !row.end_time.trim().is_empty());

    !row.site_id.trim().is_empty()
        && !row.event_id.trim().is_empty()
        && !row.source.trim().is_empty()
        && !row.source_event_id.trim().is_empty()
        && row.observation_year >= 2000
        && !row.event_type.trim().is_empty()
        && confidence_is_labeled
        && has_timing
}

