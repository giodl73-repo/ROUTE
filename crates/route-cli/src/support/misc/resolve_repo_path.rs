//! Helper `resolve_repo_path`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn resolve_repo_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() {
        return direct;
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(|workspace| workspace.join(path))
        .unwrap_or(direct)
}

