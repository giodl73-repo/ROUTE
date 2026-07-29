//! Helper `t1_diamond_validation_tasks`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_diamond_validation_tasks(
    rows: &[T1DiamondValidationRow],
    priority: Option<&str>,
    source_rows: Option<&[T1SourceHealthRow]>,
) -> Vec<T1DiamondValidationTask> {
    let source_by_site = t1_source_health_by_site(source_rows.unwrap_or(&[]));
    let mut tasks = Vec::new();
    for row in rows.iter().filter(|row| {
        priority
            .map(|priority| row.priority_band.eq_ignore_ascii_case(priority))
            .unwrap_or(true)
    }) {
        if !row.analyzer_status.eq_ignore_ascii_case("recognized") {
            tasks.push(t1_diamond_validation_task(
                row,
                "analyzer_anchor",
                "Fix analyzer recognition for the curated T1/T1 pair",
                None,
            ));
        }
        if !row.manual_geometry_status.eq_ignore_ascii_case("validated") {
            tasks.push(t1_diamond_validation_task(
                row,
                "manual_geometry",
                "Validate anchor coordinates, interchange shape, and independent transfer paths",
                None,
            ));
        }
        if !row
            .alternate_capacity_status
            .eq_ignore_ascii_case("validated")
        {
            tasks.push(t1_diamond_validation_task(
                row,
                "alternate_capacity",
                "Validate truck-capable alternate capacity and restrictions",
                None,
            ));
        }
        if !row
            .observed_failure_status
            .eq_ignore_ascii_case("empirical")
        {
            tasks.push(t1_diamond_validation_task(
                row,
                "observed_failure",
                "Attach observed closure, work-zone, duration, or incident evidence",
                source_by_site.get(row.site_id.as_str()).map(|source| {
                    format!(
                        "{} [{} / {}]: {}",
                        source.source_name,
                        source.access_health,
                        source.history_status,
                        source.next_step
                    )
                }),
            ));
        }
    }

    tasks.sort_by(|a, b| {
        t1_diamond_priority_rank(&a.priority_band)
            .cmp(&t1_diamond_priority_rank(&b.priority_band))
            .then_with(|| a.category.cmp(b.category))
            .then_with(|| a.site_id.cmp(&b.site_id))
    });
    tasks
}

