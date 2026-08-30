//! Helper `missing_moment_artifacts`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn missing_moment_artifacts(primary_artifacts: &str) -> Vec<String> {
    primary_artifacts
        .split(';')
        .map(str::trim)
        .filter(|artifact| !artifact.is_empty())
        .filter(|artifact| !moment_artifact_exists(artifact))
        .map(str::to_string)
        .collect()
}
