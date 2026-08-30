//! Helper `artifact_has_content`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn artifact_has_content(path: &str) -> bool {
    std::fs::metadata(repo_relative_artifact_path(path))
        .map(|metadata| metadata.is_file() && metadata.len() > 0)
        .unwrap_or(false)
}
