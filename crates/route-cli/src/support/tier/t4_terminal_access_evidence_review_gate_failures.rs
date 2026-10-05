//! Helper `t4_terminal_access_evidence_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_evidence_review_gate_failures(
    rows: &[T4TerminalAccessEvidenceReviewRow],
    contact_rows: &[T4TerminalContactEvidenceRow],
) -> Vec<String> {
    let expected = contact_rows
        .iter()
        .map(|row| row.queue_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("terminal contact evidence queue is empty".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "terminal access evidence review has {} rows but expected {} contact rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.review_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.terminal_district_seed.trim().is_empty()
            || row.terminal_district_seed_source.trim().is_empty()
            || row.evidence_status_before.trim().is_empty()
            || row.review_decision.trim().is_empty()
            || row.review_reason.trim().is_empty()
            || row.source_action.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete review fields", row.queue_id));
        }
        if !seen.insert(row.queue_id.clone()) {
            failures.push(format!("{} appears more than once", row.queue_id));
        }
        if !expected.contains(row.queue_id.as_str()) {
            failures.push(format!(
                "{} is not from terminal contact evidence",
                row.queue_id
            ));
        }
        if row.review_decision != "held-source-needed"
            || row.source_action != "route-to-terminal-access-proof-acquisition"
            || row.validation_status != "review"
        {
            failures.push(format!("{} promoted terminal evidence", row.queue_id));
        }
        if row.blocker_claims_before != "map;publication;upgrade"
            || row.blocker_claims_after != "map;publication;upgrade"
            || row.claim_blocker_delta != 0
        {
            failures.push(format!("{} did not preserve blockers", row.queue_id));
        }
        if row.evidence_status_before != "source-needed" {
            failures.push(format!(
                "{} review is only allowed for source-needed evidence",
                row.queue_id
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from terminal access review"));
        }
    }
    failures
}
