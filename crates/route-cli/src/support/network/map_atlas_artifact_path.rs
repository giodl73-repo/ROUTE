//! Helper `map_atlas_artifact_path`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn map_atlas_artifact_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() || direct.is_absolute() {
        direct
    } else {
        repository_root::repository_root()
            .map(|root| root.join(path))
            .unwrap_or(direct)
    }
}
