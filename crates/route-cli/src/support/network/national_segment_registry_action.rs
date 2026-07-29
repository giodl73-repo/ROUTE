//! Helper `national_segment_registry_action`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn national_segment_registry_action(
    board_layers: &std::collections::BTreeSet<String>,
    stop_statuses: &std::collections::BTreeSet<String>,
    evidence_state_scope: &std::collections::BTreeSet<String>,
    geometry_state_scope: &std::collections::BTreeSet<String>,
) -> &'static str {
    if board_layers.contains("zone-summary") || board_layers.contains("unassigned-gap-backlog") {
        return "track-zone-or-backlog-identity";
    }
    if board_layers.contains("tier-segment-candidate") {
        return "eligible-for-service-bundle";
    }
    if stop_statuses.contains("ready-for-stop-layout") {
        return "eligible-for-geometry-layout";
    }
    if evidence_state_scope.is_empty() && geometry_state_scope.is_empty() {
        return "author-zone-bounded-stop-chain";
    }
    "complete-terminal-stop-chain"
}

