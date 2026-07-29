//! Helper `t1_access_category`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_access_category(row: &T1SourceHealthRow) -> &'static str {
    match row.access_health.as_str() {
        "requires_key" => "api_key",
        "requires_account" => "account",
        "requires_access" => "access_request",
        "blocked_query" => "endpoint_tuning",
        "blocked_access" => "records_request",
        _ if row.ingestion_status != "implemented" => "implementation",
        _ if row.history_status == "snapshot_only" => "history_archive",
        _ => "monitoring",
    }
}

