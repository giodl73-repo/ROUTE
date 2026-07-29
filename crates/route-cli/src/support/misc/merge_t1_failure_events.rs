//! Helper `merge_t1_failure_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn merge_t1_failure_events(
    existing: &[T1FailureEventRow],
    incoming: &[T1FailureEventRow],
) -> Vec<T1FailureEventRow> {
    let mut rows = existing.to_vec();
    let mut seen = rows
        .iter()
        .map(t1_failure_event_key)
        .collect::<std::collections::BTreeSet<_>>();

    for row in incoming {
        if seen.insert(t1_failure_event_key(row)) {
            rows.push(row.clone());
        }
    }

    rows.sort_by(|a, b| {
        a.site_id
            .cmp(&b.site_id)
            .then_with(|| a.observation_year.cmp(&b.observation_year))
            .then_with(|| a.event_id.cmp(&b.event_id))
    });
    rows
}

