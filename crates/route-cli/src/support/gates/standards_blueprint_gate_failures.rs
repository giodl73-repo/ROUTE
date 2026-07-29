//! Helper `standards_blueprint_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn standards_blueprint_gate_failures(rows: &[StandardsProofRow]) -> Vec<&StandardsProofRow> {
    rows.iter()
        .filter(|row| {
            !standards_evidence_level_is_allowed(&row.evidence_level)
                || !row.evidence_level.eq_ignore_ascii_case("Implemented")
                || !row.blocking_gap.trim().is_empty()
        })
        .collect()
}

