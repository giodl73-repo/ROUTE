//! Helper `standards_pressure_row_has_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn standards_pressure_row_has_contract(row: &StandardsProofRow) -> bool {
    !row.standard_id.trim().is_empty()
        && !row.tier.trim().is_empty()
        && !row.standard_family.trim().is_empty()
        && !row.standard.trim().is_empty()
        && !row.outcome.trim().is_empty()
        && !row.mechanism.trim().is_empty()
        && !row.primary_stressor.trim().is_empty()
        && !row.acceptance_gate.trim().is_empty()
        && standards_evidence_level_is_allowed(&row.evidence_level)
        && !row.current_artifact.trim().is_empty()
        && !row.blocking_gap.trim().is_empty()
        && !row.next_command_or_test.trim().is_empty()
        && !row.owner_track.trim().is_empty()
}
