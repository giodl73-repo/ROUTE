//! Helper `t1_access_priority`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_access_priority(row: &T1SourceHealthRow) -> &'static str {
    if row.source_kind == "travel_time_reliability" {
        "critical"
    } else if row.access_health == "blocked_query" || row.access_health == "blocked_access" {
        "high"
    } else if row.access_health == "requires_access" || row.access_health == "requires_key" {
        "high"
    } else {
        "medium"
    }
}

