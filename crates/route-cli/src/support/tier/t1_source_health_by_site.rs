//! Helper `t1_source_health_by_site`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_source_health_by_site(
    rows: &[T1SourceHealthRow],
) -> std::collections::HashMap<&str, &T1SourceHealthRow> {
    let mut by_site = std::collections::HashMap::new();
    for row in rows {
        by_site.entry(row.site_id.as_str()).or_insert(row);
    }
    by_site
}

