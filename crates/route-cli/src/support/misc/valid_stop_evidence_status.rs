//! Helper `valid_stop_evidence_status`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn valid_stop_evidence_status(status: &str) -> bool {
    matches!(
        status.trim().to_ascii_lowercase().as_str(),
        "validated"
            | "heuristic"
            | "planned"
            | "partial"
            | "source_needed"
            | "missing_source"
            | "missing_graph_data"
    )
}

