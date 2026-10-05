//! Helper `t4_terminal_contact_proof_artifact_contract_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_proof_artifact_contract_rows(
) -> Vec<T4TerminalContactProofArtifactContractRow> {
    vec![T4TerminalContactProofArtifactContractRow {
        contract_id: "T4CONTACT-PROOF-CONTRACT-001".to_string(),
        source_family: "public-terminal-contact-proof".to_string(),
        accepted_proof_status: "source-backed".to_string(),
        required_fields:
            "route; terminal district; route-to-terminal contact statement; source title; source url or cached artifact; capture date; selected higher-tier attachment; validation decision"
                .to_string(),
        allowed_artifact_modes: "manual-citation;cached-source-artifact".to_string(),
        prohibited_sources: "data/intermodal_terminals.csv;terminal district seed;route proximity;district membership"
            .to_string(),
        promotion_rule:
            "source-backed requires a non-seed source artifact naming route terminal district contact statement source title url-or-cache capture date selected higher-tier attachment and validation decision"
                .to_string(),
        source_needed_decision:
            "missing proof artifact remains source-needed and review".to_string(),
        blocked_decision:
            "inaccessible or policy-unsupported source remains blocked with blocker text".to_string(),
        rejected_decision:
            "artifact that does not name route-to-terminal contact remains rejected and cannot feed scenario readiness"
                .to_string(),
        next_artifact:
            "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-02.md"
                .to_string(),
        validation_status: "pass".to_string(),
    }]
}
