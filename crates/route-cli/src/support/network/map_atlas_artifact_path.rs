//! Helper `map_atlas_artifact_path`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn map_atlas_artifact_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() || direct.is_absolute() {
        direct
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path)
    }
}

