//! Helper `repo_relative_artifact_path`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn repo_relative_artifact_path(path: &str) -> PathBuf {
    let direct = std::path::PathBuf::from(path);
    if direct.exists() || direct.is_absolute() {
        direct
    } else {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path)
    }
}

