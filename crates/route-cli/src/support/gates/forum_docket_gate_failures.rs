//! Helper `forum_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn forum_docket_gate_failures(rows: &[ForumDocketRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["forum docket has no review rows".to_string()];
    }

    rows.iter()
        .filter_map(|row| forum_docket_row_failure(row))
        .collect()
}
