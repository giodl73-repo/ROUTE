//! Helper `blueprint_evidence_row_failure`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn blueprint_evidence_row_failure(
    row: &BlueprintEvidenceRow,
    package_ids: &std::collections::HashSet<&str>,
    standard_evidence: &std::collections::HashMap<&str, &str>,
) -> Option<String> {
    let claim_status = row.blueprint_claim_status.trim().to_ascii_lowercase();
    let claim_status_ok = matches!(
        claim_status.as_str(),
        "implemented" | "heuristic" | "planned" | "held" | "downgraded"
    );
    let standard_id = row.standard_id.trim();
    let expected_evidence = standard_evidence.get(standard_id).copied();
    let evidence_matches = expected_evidence
        .map(|expected| expected.trim() == row.proof_evidence_level.trim())
        .unwrap_or(false);
    let no_premature_promotion = if standards_evidence_level_is_allowed(&row.proof_evidence_level) {
        row.proof_evidence_level
            .trim()
            .eq_ignore_ascii_case("Implemented")
            || !claim_status.eq("implemented")
    } else {
        false
    };
    let filled = !row.package_id.trim().is_empty()
        && package_ids.contains(row.package_id.trim())
        && !standard_id.is_empty()
        && expected_evidence.is_some()
        && evidence_matches
        && claim_status_ok
        && no_premature_promotion
        && !row.promotion_rule.trim().is_empty()
        && !row.proof_artifact.trim().is_empty()
        && !row.forum_hold.trim().is_empty()
        && !row.blocking_gap.trim().is_empty()
        && !row.required_next_evidence.trim().is_empty();

    if filled {
        None
    } else {
        Some(format!(
            "{}:{} invalid evidence map row: proof={} claim={}",
            if row.package_id.trim().is_empty() {
                "<missing-package-id>"
            } else {
                row.package_id.as_str()
            },
            if standard_id.is_empty() {
                "<missing-standard-id>"
            } else {
                row.standard_id.as_str()
            },
            row.proof_evidence_level,
            row.blueprint_claim_status
        ))
    }
}

