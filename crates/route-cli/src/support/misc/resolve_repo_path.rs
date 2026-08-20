//! Helper `resolve_repo_path`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn resolve_repo_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() {
        return direct;
    }
    repository_root::repository_root()
        .map(|root| root.join(path))
        .unwrap_or(direct)
}
