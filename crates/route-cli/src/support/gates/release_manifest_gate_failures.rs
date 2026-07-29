//! Helper `release_manifest_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn release_manifest_gate_failures(rows: &[ReleaseManifestRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["release manifest has no rows".to_string()];
    }
    let allowed_release_status = ["release_candidate", "planned", "held", "retired"];
    let allowed_public_status = ["public", "held_public", "internal", "source_needed"];
    let mut seen_paths = std::collections::HashSet::new();
    let mut failures = Vec::new();

    for row in rows {
        let path = row.artifact_path.trim();
        if path.is_empty()
            || row.artifact_class.trim().is_empty()
            || row.owner_milepost.trim().is_empty()
            || row.release_status.trim().is_empty()
            || row.public_status.trim().is_empty()
            || row.verification_command.trim().is_empty()
            || row.notes.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete release manifest fields",
                if path.is_empty() {
                    "<missing-path>"
                } else {
                    path
                }
            ));
            continue;
        }
        if !seen_paths.insert(path.to_string()) {
            failures.push(format!("{path} duplicate release manifest artifact"));
        }
        if !release_manifest_artifact_exists(path) {
            failures.push(format!("{path} release artifact is missing"));
        }
        if !release_manifest_verification_command_allowed(row) {
            failures.push(format!(
                "{path} has unsupported verification command {}",
                row.verification_command
            ));
        }
        if !row.owner_milepost.starts_with('M') || row.owner_milepost[1..].parse::<u8>().is_err() {
            failures.push(format!(
                "{path} has invalid owner milepost {}",
                row.owner_milepost
            ));
        }
        if !allowed_release_status.contains(&row.release_status.as_str()) {
            failures.push(format!(
                "{path} has invalid release status {}",
                row.release_status
            ));
        }
        if !allowed_public_status.contains(&row.public_status.as_str()) {
            failures.push(format!(
                "{path} has invalid public status {}",
                row.public_status
            ));
        }
        if row.public_status == "public" && row.release_status == "held" {
            failures.push(format!("{path} cannot be public while release-held"));
        }
    }

    for required in [
        "data/release-manifest.csv",
        "docs/SPEC_INDEX.md",
        "data/source-fetch-policy.csv",
        "docs/source-fetch-cache-policy.md",
        "docs/optimizer-artifact-manifest.md",
        "data/tier-optimizer-runs.csv",
    ] {
        if !seen_paths.contains(required) {
            failures.push(format!("missing release manifest row for {required}"));
        }
    }
    if seen_paths.contains("data/tier-optimizer-runs.csv") {
        failures.extend(release_manifest_optimizer_bundle_failures());
    }
    failures
}

