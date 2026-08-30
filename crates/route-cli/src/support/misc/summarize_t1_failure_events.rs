//! Helper `summarize_t1_failure_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn summarize_t1_failure_events(
    rows: &[T1FailureEventRow],
) -> Vec<T1FailureEventSummary> {
    let mut by_site: std::collections::BTreeMap<&str, Vec<&T1FailureEventRow>> =
        std::collections::BTreeMap::new();
    for row in rows.iter().filter(|row| row.freight_relevant) {
        by_site.entry(&row.site_id).or_default().push(row);
    }

    by_site
        .into_iter()
        .map(|(site_id, site_rows)| {
            let mut years = site_rows
                .iter()
                .map(|row| row.observation_year)
                .collect::<Vec<_>>();
            years.sort_unstable();
            years.dedup();

            let mut event_ids = site_rows
                .iter()
                .map(|row| row.event_id.as_str())
                .collect::<Vec<_>>();
            event_ids.sort_unstable();
            event_ids.dedup();

            let mut durations = site_rows
                .iter()
                .filter_map(|row| row.duration_hours)
                .filter(|v| v.is_finite() && *v >= 0.0)
                .collect::<Vec<_>>();
            durations.sort_by(|a, b| a.total_cmp(b));

            let observed_years = years.len();
            let event_count = event_ids.len();
            let annual_rate = if observed_years > 0 {
                event_count as f64 / observed_years as f64
            } else {
                0.0
            };
            let confidence = event_summary_confidence(&site_rows);

            T1FailureEventSummary {
                site_id: site_id.to_string(),
                observed_years,
                event_count,
                annual_rate,
                annual_probability: annual_probability_from_rate(annual_rate),
                duration_p50_hours: percentile_nearest(&durations, 0.50),
                duration_p95_hours: percentile_nearest(&durations, 0.95),
                confidence,
            }
        })
        .collect()
}
