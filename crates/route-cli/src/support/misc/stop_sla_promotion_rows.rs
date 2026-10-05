//! Helper `stop_sla_promotion_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_sla_promotion_rows(
    docket: &[StopSlaCandidateDocketRow],
    include_ledger: bool,
    include_alternates: bool,
) -> Vec<StopCandidateRow> {
    let mut seen_gaps = std::collections::BTreeSet::new();
    let mut rows = Vec::new();
    for row in docket {
        if !include_ledger && row.candidate_source_type == "stop-ledger" {
            continue;
        }
        if !include_alternates && !seen_gaps.insert(row.gap_segment.clone()) {
            continue;
        }
        rows.push(stop_sla_promotion_row(row));
    }
    rows
}
