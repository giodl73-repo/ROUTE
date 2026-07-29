//! Helper `significant_moment_row_failure`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn significant_moment_row_failure(row: &SignificantMomentRow) -> Option<String> {
    let id = row.moment_id.trim();
    let required_filled = !id.is_empty()
        && !row.date.trim().is_empty()
        && !row.flair.trim().is_empty()
        && !row.kind.trim().is_empty()
        && !row.summary.trim().is_empty()
        && !row.why_it_mattered.trim().is_empty()
        && !row.primary_artifacts.trim().is_empty()
        && !row.commit.trim().is_empty()
        && !row.next_thread.trim().is_empty();
    let id_ok = id.starts_with("MOM-");
    let date_ok = looks_like_iso_date(row.date.trim());
    let kind_ok = row.kind.trim().ends_with("_breakthrough");
    let commit_ok = looks_like_commit_ref(row.commit.trim());
    let missing_artifacts = missing_moment_artifacts(&row.primary_artifacts);

    if !required_filled
        || !id_ok
        || !date_ok
        || !kind_ok
        || !commit_ok
        || !missing_artifacts.is_empty()
    {
        let artifact_note = if missing_artifacts.is_empty() {
            "none".to_string()
        } else {
            missing_artifacts.join("; ")
        };
        Some(format!(
            "{} invalid moment contract: date={} kind={} commit={} missing_artifacts={}",
            if id.is_empty() {
                "<missing-moment-id>"
            } else {
                id
            },
            row.date,
            row.kind,
            row.commit,
            artifact_note
        ))
    } else {
        None
    }
}

