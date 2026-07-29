//! Helper `t1_source_health_is_blocked`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_source_health_is_blocked(row: &T1SourceHealthRow) -> bool {
    !matches!(
        (
            row.access_health.as_str(),
            row.ingestion_status.as_str(),
            row.history_status.as_str()
        ),
        ("live", "implemented", "snapshot_only") | ("live", "documented", "historical_method")
    )
}

