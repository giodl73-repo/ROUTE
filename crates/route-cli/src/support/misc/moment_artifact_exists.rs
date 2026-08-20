//! Helper `moment_artifact_exists`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn moment_artifact_exists(artifact: &str) -> bool {
    let path = Path::new(artifact);
    if path.exists() {
        return true;
    }
    repository_root::repository_root()
        .map(|root| root.join(path).exists())
        .unwrap_or(false)
}
