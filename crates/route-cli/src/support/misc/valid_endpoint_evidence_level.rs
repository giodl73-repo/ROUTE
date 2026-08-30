//! Helper `valid_endpoint_evidence_level`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn valid_endpoint_evidence_level(level: &str) -> bool {
    matches!(
        level.trim().to_ascii_lowercase().as_str(),
        "validated" | "heuristic" | "planned" | "missing_graph_data" | "demote"
    )
}
