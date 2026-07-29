//! Helper `forum_docket_row_failure`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn forum_docket_row_failure(row: &ForumDocketRow) -> Option<String> {
    let review_type = row.review_type.trim().to_ascii_lowercase();
    let status = row.status.trim().to_ascii_lowercase();
    let type_ok = matches!(
        review_type.as_str(),
        "parliament" | "stakeholder" | "editorial" | "panel" | "owner"
    );
    let status_ok = matches!(status.as_str(), "planned" | "complete" | "held");
    let required_filled = !row.review_id.trim().is_empty()
        && !row.artifact.trim().is_empty()
        && !row.roles.trim().is_empty()
        && !row.claim_target.trim().is_empty()
        && !row.blocking_question.trim().is_empty()
        && !row.next_action.trim().is_empty()
        && !row.output_artifact.trim().is_empty();

    if !type_ok || !status_ok || !required_filled {
        Some(format!(
            "{} invalid contract: type={} status={} artifact={} output={}",
            if row.review_id.trim().is_empty() {
                "<missing-review-id>"
            } else {
                row.review_id.as_str()
            },
            row.review_type,
            row.status,
            row.artifact,
            row.output_artifact
        ))
    } else {
        None
    }
}

