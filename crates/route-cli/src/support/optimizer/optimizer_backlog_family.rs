//! Helper `optimizer_backlog_family`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_backlog_family(row: &OptimizerConstraintBudgetRow) -> (String, String, String) {
    let classes = row.top_constraint_classes.as_str();
    if row.hard_blocker_count > 0 {
        return (
            "P0-hard-blocker".to_string(),
            classes.to_string(),
            "hard-blocker-resolution".to_string(),
        );
    }
    if row.claim_blocker_count > 0 && classes.contains("game_ops_bundle_binding") {
        return (
            "P1-game-claim".to_string(),
            "game_ops_bundle_binding".to_string(),
            "game-ops-blocker-evidence-review".to_string(),
        );
    }
    if row.claim_blocker_count > 0 && classes.contains("terminal_access_evidence_gap") {
        return (
            "P1-terminal-evidence".to_string(),
            "terminal_access_evidence_gap".to_string(),
            "terminal-access-evidence-review".to_string(),
        );
    }
    if classes.contains("asset_condition_debt") {
        return (
            "P2-asset-debt".to_string(),
            "asset_condition_debt".to_string(),
            "asset-condition-debt-repair".to_string(),
        );
    }
    if row.claim_blocker_count > 0
        && (classes.contains("terminal_contact") || classes.contains("source"))
    {
        return (
            "P2-source-evidence".to_string(),
            classes.to_string(),
            "source-evidence-acquisition".to_string(),
        );
    }
    if row.claim_blocker_count > 0 {
        return (
            "P1-claim-blocker".to_string(),
            classes.to_string(),
            "optimizer-claim-review".to_string(),
        );
    }
    (
        "P3-review-debt".to_string(),
        classes.to_string(),
        "optimizer-review-docket".to_string(),
    )
}

