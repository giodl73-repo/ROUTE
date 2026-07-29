//! Helper `source_fetch_policy_row`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn source_fetch_policy_row(
    fetch_family: &str,
    commands: &str,
    cache_targets: &str,
    mutation_mode: &str,
    preservation_contract: &str,
    implementation_guard: &str,
    validation_floor: &str,
) -> SourceFetchPolicyRow {
    SourceFetchPolicyRow {
        fetch_family: fetch_family.to_string(),
        commands: commands.to_string(),
        cache_targets: cache_targets.to_string(),
        mutation_mode: mutation_mode.to_string(),
        preservation_contract: preservation_contract.to_string(),
        implementation_guard: implementation_guard.to_string(),
        validation_floor: validation_floor.to_string(),
        policy_doc: "docs/source-fetch-cache-policy.md".to_string(),
        validation_status: "pass".to_string(),
    }
}

