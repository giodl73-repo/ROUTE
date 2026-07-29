//! Helper `t4_terminal_contact_evidence_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_evidence_gate_failures(
    rows: &[T4TerminalContactEvidenceRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T4 terminal contact evidence rows emitted".to_string());
        return failures;
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.terminal_district_seed.trim().is_empty()
            || row.terminal_district_seed_source.trim().is_empty()
            || row.contact_basis.trim().is_empty()
            || row.evidence_status.trim().is_empty()
            || row.selected_higher_tier_attachment.trim().is_empty()
            || row.decision.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete terminal contact evidence fields",
                row.queue_id
            ));
        }
        if !seen.insert(row.queue_id.clone()) {
            failures.push(format!("{} is duplicated", row.queue_id));
        }
        if row.terminal_district_seed_source == row.contact_proof_source
            && !row.contact_proof_source.trim().is_empty()
        {
            failures.push(format!(
                "{} uses terminal district seed source as contact proof",
                row.queue_id
            ));
        }
        if !matches!(
            row.decision.as_str(),
            "source-needed"
                | "source-backed"
                | "demotion/local-only"
                | "held-known"
                | "scenario-ready"
        ) {
            failures.push(format!(
                "{} has invalid decision {}",
                row.queue_id, row.decision
            ));
        }
        if !matches!(
            row.evidence_status.as_str(),
            "source-needed" | "accepted" | "held-known" | "demoted"
        ) {
            failures.push(format!(
                "{} has invalid evidence status {}",
                row.queue_id, row.evidence_status
            ));
        }
        if row.decision == "source-needed" {
            if row.evidence_status != "source-needed" || !row.contact_proof_source.trim().is_empty()
            {
                failures.push(format!(
                    "{} source-needed decision has contact proof or non-source-needed evidence",
                    row.queue_id
                ));
            }
            if row.validation_status != "review" {
                failures.push(format!(
                    "{} source-needed row must remain review",
                    row.queue_id
                ));
            }
        }
        if matches!(row.decision.as_str(), "source-backed" | "scenario-ready")
            && (row.evidence_status != "accepted"
                || row.contact_proof_source.trim().is_empty()
                || row.selected_higher_tier_attachment.trim().is_empty()
                || row.selected_higher_tier_attachment == "source-needed")
        {
            failures.push(format!(
                "{} source-backed/scenario-ready decision lacks proof or attachment",
                row.queue_id
            ));
        }
        if row.decision == "scenario-ready"
            && row.contact_basis.to_ascii_lowercase().contains("proximity")
        {
            failures.push(format!(
                "{} proximity-only contact cannot be scenario-ready",
                row.queue_id
            ));
        }
        if row.decision == "demotion/local-only" && row.evidence_status != "demoted" {
            failures.push(format!(
                "{} demotion/local-only decision must use demoted evidence status",
                row.queue_id
            ));
        }
        if row.decision == "held-known" && row.evidence_status != "held-known" {
            failures.push(format!(
                "{} held-known decision must use held-known evidence status",
                row.queue_id
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review" | "held") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.queue_id, row.validation_status
            ));
        }
    }
    failures
}

