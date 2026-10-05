//! Helper `release_manifest_optimizer_bundle_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn release_manifest_optimizer_bundle_failures() -> Vec<String> {
    let manifest_path = release_manifest_artifact_path("data/tier-optimizer-runs.csv");
    let rows = match load_tier_optimizer_runs(&manifest_path) {
        Ok(rows) => rows,
        Err(error) => {
            return vec![format!(
                "data/tier-optimizer-runs.csv could not be loaded for release coverage: {error}"
            )];
        }
    };
    rows.iter()
        .filter(|row| matches!(row.gate_status.as_str(), "pass" | "held-known"))
        .filter(|row| !release_manifest_artifact_exists(&row.artifact))
        .map(|row| {
            format!(
                "{} optimizer artifact is missing from release coverage bundle",
                row.artifact
            )
        })
        .collect()
}
