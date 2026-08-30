//! Helper `t4_terminal_contact_proof_artifact_contract_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_proof_artifact_contract_gate_failures(
    rows: &[T4TerminalContactProofArtifactContractRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no terminal contact proof artifact contract rows emitted".to_string());
        return failures;
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.contract_id.trim().is_empty()
            || row.source_family.trim().is_empty()
            || row.accepted_proof_status.trim().is_empty()
            || row.required_fields.trim().is_empty()
            || row.allowed_artifact_modes.trim().is_empty()
            || row.prohibited_sources.trim().is_empty()
            || row.promotion_rule.trim().is_empty()
            || row.source_needed_decision.trim().is_empty()
            || row.blocked_decision.trim().is_empty()
            || row.rejected_decision.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete contract fields",
                row.contract_id
            ));
        }
        if !seen.insert(row.contract_id.clone()) {
            failures.push(format!("{} appears more than once", row.contract_id));
        }
        if row.source_family != "public-terminal-contact-proof" {
            failures.push(format!(
                "{} has unsupported source family {}",
                row.contract_id, row.source_family
            ));
        }
        if row.accepted_proof_status != "source-backed" {
            failures.push(format!(
                "{} does not define source-backed acceptance",
                row.contract_id
            ));
        }
        for required in [
            "route",
            "terminal district",
            "route-to-terminal contact statement",
            "source title",
            "source url or cached artifact",
            "capture date",
            "selected higher-tier attachment",
            "validation decision",
        ] {
            if !row.required_fields.contains(required) {
                failures.push(format!(
                    "{} missing required proof field {}",
                    row.contract_id, required
                ));
            }
        }
        for mode in ["manual-citation", "cached-source-artifact"] {
            if !row.allowed_artifact_modes.contains(mode) {
                failures.push(format!(
                    "{} missing allowed artifact mode {}",
                    row.contract_id, mode
                ));
            }
        }
        for prohibited in [
            "data/intermodal_terminals.csv",
            "terminal district seed",
            "route proximity",
            "district membership",
        ] {
            if !row.prohibited_sources.contains(prohibited) {
                failures.push(format!(
                    "{} missing prohibited source {}",
                    row.contract_id, prohibited
                ));
            }
        }
        if !row.promotion_rule.contains("non-seed source artifact")
            || !row
                .promotion_rule
                .contains("route terminal district contact statement")
        {
            failures.push(format!(
                "{} promotion rule does not require non-seed route contact proof",
                row.contract_id
            ));
        }
        if !row.source_needed_decision.contains("source-needed")
            || !row.blocked_decision.contains("blocked")
            || !row.rejected_decision.contains("rejected")
        {
            failures.push(format!(
                "{} does not preserve unresolved/rejected decision states",
                row.contract_id
            ));
        }
        if row.validation_status != "pass" {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.contract_id, row.validation_status
            ));
        }
    }
    failures
}
