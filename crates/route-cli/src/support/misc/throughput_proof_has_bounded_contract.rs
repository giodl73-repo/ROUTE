//! Helper `throughput_proof_has_bounded_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn throughput_proof_has_bounded_contract(row: &ThroughputProofRow) -> bool {
    let binding = row.binding_type.trim().to_ascii_lowercase();
    let binding_is_labeled = matches!(
        binding.as_str(),
        "congestion_binding" | "resilience_binding"
    );
    row.proof_id.starts_with("TP-")
        && !row.proof_name.trim().is_empty()
        && binding_is_labeled
        && !row.stressor.trim().is_empty()
        && !row.primary_metric.trim().is_empty()
        && !row.existing_artifact.trim().is_empty()
        && standards_evidence_level_is_allowed(&row.current_status)
        && !row.next_evidence_step.trim().is_empty()
}

