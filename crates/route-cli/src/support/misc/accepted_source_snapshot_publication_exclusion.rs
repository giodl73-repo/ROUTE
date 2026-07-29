//! Helper `accepted_source_snapshot_publication_exclusion`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn accepted_source_snapshot_publication_exclusion(
    rows: &[SourceSnapshotPublicationExclusionRow],
) -> Option<&SourceSnapshotPublicationExclusionRow> {
    rows.iter().find(|row| {
        row.decision == "exclude-live-snapshot-guard-from-map-publication"
            && row.validation_status == "accepted"
            && row.affected_constraint_class == "source_acquisition_snapshot_guard"
            && row.affected_fetch_family == "t1-live-event-snapshots"
            && row.excluded_claims == "publication"
            && row.preserved_claims_after == "evidence"
    })
}

