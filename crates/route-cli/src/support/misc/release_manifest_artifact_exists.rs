//! Helper `release_manifest_artifact_exists`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn release_manifest_artifact_exists(path: &str) -> bool {
    repo_relative_artifact_path(path).exists()
}

