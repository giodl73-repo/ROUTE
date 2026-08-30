//! Helper `significant_moment_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn significant_moment_gate_failures(rows: &[SignificantMomentRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["significant moment ledger has no rows".to_string()];
    }

    let mut seen_ids = std::collections::HashSet::new();
    let mut seen_flairs = std::collections::HashSet::new();
    let mut failures = Vec::new();

    for row in rows {
        if let Some(failure) = significant_moment_row_failure(row) {
            failures.push(failure);
        }

        let id = row.moment_id.trim();
        if !id.is_empty() && !seen_ids.insert(id.to_string()) {
            failures.push(format!("{id} duplicate moment_id"));
        }

        let flair = row.flair.trim().to_ascii_lowercase();
        if !flair.is_empty() && !seen_flairs.insert(flair) {
            failures.push(format!("{} duplicate flair '{}'", row.moment_id, row.flair));
        }
    }

    failures
}
