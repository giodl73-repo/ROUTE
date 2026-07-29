//! Helper `moment_artifact_exists`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn moment_artifact_exists(artifact: &str) -> bool {
    let path = Path::new(artifact);
    if path.exists() {
        return true;
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
        .exists()
}

