//! Helper `tier_contact_witness_is_unresolved_blocker`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_contact_witness_is_unresolved_blocker(
    witness_type: &str,
    required_artifact: &str,
    validation_status: &str,
) -> bool {
    if validation_status.eq_ignore_ascii_case("pass") {
        return false;
    }
    matches!(
        witness_type,
        "dual-contact-needed"
            | "parent-contact-needed"
            | "graph-contact-needed"
            | "unknown-repair-action"
    ) && !matches!(
        required_artifact,
        "data/tier-candidate-columns.csv" | "data/tier-table.csv"
    )
}
