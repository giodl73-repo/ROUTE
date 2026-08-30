//! Helper `source_fetch_policy_row_covers_command`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn source_fetch_policy_row_covers_command(
    row: &SourceFetchPolicyRow,
    command: &str,
) -> bool {
    row.commands
        .split(';')
        .map(str::trim)
        .any(|candidate| candidate == command || candidate.starts_with(&format!("{command} ")))
}
