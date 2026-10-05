//! Helper `release_manifest_verification_command_allowed`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn release_manifest_verification_command_allowed(row: &ReleaseManifestRow) -> bool {
    let command = row.verification_command.trim();
    if command == "manual review" {
        return release_manifest_manual_review_allowed(row);
    }
    if command == "cargo test --workspace" {
        return true;
    }
    if command.starts_with("powershell -ExecutionPolicy Bypass -File ") {
        return true;
    }
    let route_args = command
        .strip_prefix("cargo run -q -p route -- ")
        .or_else(|| command.strip_prefix("route "));
    let Some(route_args) = route_args else {
        return false;
    };
    let parts: Vec<&str> = route_args.split_whitespace().collect();
    if parts.iter().any(|part| part.starts_with("--gate")) {
        return true;
    }
    matches!(
        parts.as_slice(),
        ["score-all"] | ["beck-t1-diagnostics"] | ["gap", "--type", _]
    )
}
