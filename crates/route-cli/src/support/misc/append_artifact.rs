//! Helper `append_artifact`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn append_artifact(existing: &str, artifact: &Path) -> String {
    let artifact = artifact.to_string_lossy();
    if existing
        .split(';')
        .map(str::trim)
        .any(|value| value == artifact)
    {
        existing.to_string()
    } else if existing.trim().is_empty() {
        artifact.to_string()
    } else {
        format!("{}; {}", existing.trim(), artifact)
    }
}

