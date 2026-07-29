//! Helper `standards_evidence_level_is_allowed`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn standards_evidence_level_is_allowed(level: &str) -> bool {
    matches!(
        level.trim().to_ascii_lowercase().as_str(),
        "implemented" | "heuristic" | "stub" | "planned" | "deprecated"
    )
}

