//! Helper `t4_terminal_columbus_proof_attempt_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_columbus_proof_attempt_gate_failures(
    rows: &[T4TerminalColumbusProofAttemptRow],
    source_access_rows: &[T4TerminalColumbusSourceAccessRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_ids = source_access_rows
        .iter()
        .map(|row| row.queue_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    if rows.is_empty() {
        failures.push("no Columbus South proof attempt rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected_ids.len() {
        failures.push(format!(
            "Columbus proof attempts have {} rows but expected {} source access rows",
            rows.len(),
            expected_ids.len()
        ));
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.attempt_id.trim().is_empty()
            || row.access_id.trim().is_empty()
            || row.intake_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.terminal_district.trim().is_empty()
            || row.source_family.trim().is_empty()
            || row.source_artifact.trim().is_empty()
            || row.capture_status.trim().is_empty()
            || row.contact_statement_status.trim().is_empty()
            || row.selected_higher_tier_attachment_status.trim().is_empty()
            || row.proof_attempt_status.trim().is_empty()
            || row.proof_decision.trim().is_empty()
            || row.proof_blocker.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete proof attempt fields",
                row.attempt_id
            ));
        }
        if !seen.insert(row.queue_id.clone()) {
            failures.push(format!(
                "{} appears more than once in Columbus proof attempts",
                row.queue_id
            ));
        }
        if row.terminal_district != "Columbus South" {
            failures.push(format!(
                "{} is not a Columbus South proof attempt row",
                row.queue_id
            ));
        }
        if !expected_ids.contains(row.queue_id.as_str()) {
            failures.push(format!(
                "{} does not appear in Columbus source access",
                row.queue_id
            ));
        }
        if !matches!(
            row.proof_attempt_status.as_str(),
            "accepted" | "source-needed" | "blocked" | "rejected"
        ) {
            failures.push(format!(
                "{} has invalid proof attempt status {}",
                row.queue_id, row.proof_attempt_status
            ));
        }
        if row.proof_attempt_status == "accepted" {
            if row.source_artifact == "source-needed"
                || row.contact_statement_status != "source-backed"
                || row.selected_higher_tier_attachment_status != "attached"
                || row.proof_decision != "source-backed"
                || row.validation_status != "pass"
            {
                failures.push(format!(
                    "{} accepted proof attempt lacks non-seed proof evidence",
                    row.queue_id
                ));
            }
        } else if row.source_artifact != "source-needed"
            || row.contact_statement_status != "source-needed"
            || row.selected_higher_tier_attachment_status != "source-needed"
            || row.validation_status != "review"
        {
            failures.push(format!(
                "{} unresolved proof attempt must remain source-needed/review",
                row.queue_id
            ));
        }
        if row.proof_attempt_status == "blocked"
            && !row
                .proof_blocker
                .contains("no safe live terminal-contact fetch command")
        {
            failures.push(format!(
                "{} blocked proof attempt lacks source-access blocker",
                row.queue_id
            ));
        }
    }

    for expected_id in expected_ids {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} is missing from Columbus proof attempts"
            ));
        }
    }

    failures
}

