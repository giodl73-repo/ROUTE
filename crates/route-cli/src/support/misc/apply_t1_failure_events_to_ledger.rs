//! Helper `apply_t1_failure_events_to_ledger`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn apply_t1_failure_events_to_ledger(
    ledger_rows: &[T1FailureRow],
    event_rows: &[T1FailureEventRow],
    event_artifact: &Path,
) -> Vec<T1FailureRow> {
    let summaries = summarize_t1_failure_events(event_rows)
        .into_iter()
        .map(|summary| (summary.site_id.clone(), summary))
        .collect::<std::collections::BTreeMap<_, _>>();

    ledger_rows
        .iter()
        .cloned()
        .map(|mut row| {
            if let Some(summary) = summaries.get(&row.site_id) {
                row.annual_probability = Some(summary.annual_probability);
                row.duration_p50_hours = summary.duration_p50_hours;
                row.duration_p95_hours = summary.duration_p95_hours;
                row.source_status = "empirical".to_string();
                row.confidence = summary.confidence.clone();
                row.current_artifact = append_artifact(&row.current_artifact, event_artifact);
                row.blocking_gap = "Snapshot empirical event observations loaded, but annual closure probability is not stable until a polling/archive window is built; reroute time and throughput retention still require source validation".to_string();
                row.next_evidence_step = "Join event windows to NPMRDS/FPM travel-time traces and reroute simulations; continue polling or obtain DOT history before publication".to_string();
            }
            row
        })
        .collect()
}
