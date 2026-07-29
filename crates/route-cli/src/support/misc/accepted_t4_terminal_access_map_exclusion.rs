//! Helper `accepted_t4_terminal_access_map_exclusion`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn accepted_t4_terminal_access_map_exclusion(
    rows: &[T4TerminalAccessMapExclusionRow],
) -> Option<&T4TerminalAccessMapExclusionRow> {
    rows.iter().find(|row| {
        row.decision == "exclude-terminal-access-overlay-from-map-publication"
            && row.validation_status == "accepted"
            && row.affected_constraint_class == "terminal_access_evidence_gap"
            && row.affected_gap_class == "terminal-evidence-needed"
            && row.affected_tier == "T4"
            && row.excluded_claims == "map|publication"
            && !row.preserved_claims_after.trim().is_empty()
    })
}

