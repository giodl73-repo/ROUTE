//! Helper `pavement_source_gap_still_open_for_task`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pavement_source_gap_still_open_for_task(
    docket_row: &TierPavementAcquisitionDocketRow,
    source_gap_rows: &[TierPavementSourceGapRow],
) -> bool {
    let affected_bundles = docket_row
        .affected_bundles
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    source_gap_rows.iter().any(|gap_row| {
        affected_bundles.contains(gap_row.segment_bundle_id.as_str())
            && gap_row
                .affected_states
                .split(';')
                .map(str::trim)
                .any(|state| state == docket_row.state)
    })
}
