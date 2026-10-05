//! Helper `t1_access_docket_item`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_access_docket_item(row: &T1SourceHealthRow) -> T1AccessDocketItem {
    let category = t1_access_category(row).to_string();
    let priority = t1_access_priority(row).to_string();
    let action = match category.as_str() {
        "api_key" => format!("Request credentials; then implement {}", row.source_name),
        "account" => format!("Obtain account/export; then map {}", row.source_name),
        "access_request" => format!(
            "Request data access or partner extract for {}",
            row.source_name
        ),
        "endpoint_tuning" => format!("Tune query/export path for {}", row.source_name),
        "records_request" => format!(
            "Request archive/export or identify allowed endpoint for {}",
            row.source_name
        ),
        _ => row.next_step.clone(),
    };
    T1AccessDocketItem {
        site_id: row.site_id.clone(),
        source_name: row.source_name.clone(),
        source_url: row.source_url.clone(),
        access_health: row.access_health.clone(),
        history_status: row.history_status.clone(),
        blocking_gap: row.blocking_gap.clone(),
        category,
        priority,
        action,
    }
}
