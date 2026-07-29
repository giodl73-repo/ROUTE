//! Helper `pressure_standard_coverage_focus`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pressure_standard_coverage_focus(standards: &[StandardsProofRow]) -> Vec<&StandardsProofRow> {
    standards
        .iter()
        .filter(|row| {
            row.tier == "T1"
                && matches!(row.standard_family.as_str(), "throughput" | "resilience")
                && !row.evidence_level.eq_ignore_ascii_case("deprecated")
        })
        .collect()
}

