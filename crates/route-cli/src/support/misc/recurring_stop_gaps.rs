//! Helper `recurring_stop_gaps`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn recurring_stop_gaps(rows: &[StopSlaRow]) -> Vec<RecurringStopGap> {
    let mut direct_pairs = std::collections::HashMap::<String, (&StopSlaRow, f64)>::new();
    for row in rows {
        let stops = row.stop_path.split(';').collect::<Vec<_>>();
        if stops.len() != 2 {
            continue;
        }
        direct_pairs.insert(
            normalized_stop_pair(&row.origin_id, &row.dest_id),
            (row, row.network_miles),
        );
    }

    let mut counts = std::collections::HashMap::<String, usize>::new();
    for row in rows {
        let stops = row.stop_path.split(';').collect::<Vec<_>>();
        for pair in stops.windows(2) {
            *counts
                .entry(normalized_stop_pair(pair[0], pair[1]))
                .or_default() += 1;
        }
    }

    let mut gaps = counts
        .into_iter()
        .filter_map(|(segment_id, row_count)| {
            let (direct, miles) = direct_pairs.get(&segment_id)?;
            Some(RecurringStopGap {
                segment_id,
                labels: format!("{} to {}", direct.origin_label, direct.dest_label),
                miles: *miles,
                row_count,
                route_path: direct.route_path.clone(),
            })
        })
        .collect::<Vec<_>>();
    gaps.sort_by(|a, b| {
        b.miles
            .total_cmp(&a.miles)
            .then_with(|| b.row_count.cmp(&a.row_count))
            .then_with(|| a.segment_id.cmp(&b.segment_id))
    });
    gaps
}
