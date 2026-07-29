//! Helper `load_release_manifest`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_release_manifest(path: &Path) -> Result<Vec<ReleaseManifestRow>> {
    let file = std::fs::File::open(path)?;
    parse_release_manifest(file)
}

