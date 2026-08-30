//! Helper `release_manifest_manual_review_allowed`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn release_manifest_manual_review_allowed(row: &ReleaseManifestRow) -> bool {
    let path = row.artifact_path.trim();
    let class = row.artifact_class.trim();
    path.starts_with("docs/")
        || path.starts_with("specs/")
        || path == "TRACKER.md"
        || path.ends_with("phase-sequence.csv")
        || class.contains("doc")
        || class.contains("plan")
        || class.contains("review")
        || class.contains("closeout")
        || class.contains("spec")
        || class.contains("standard")
        || class.contains("policy")
        || class.contains("roadmap")
        || class.contains("index")
        || class.contains("status")
        || class.contains("script")
}
