//! Helper `release_manifest_artifact_path`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn release_manifest_artifact_path(path: &str) -> PathBuf {
    repo_relative_artifact_path(path)
}
